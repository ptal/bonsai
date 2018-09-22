// Copyright 2018 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use session::*;
use context::*;
use middle::causality::causal_model::*;
use pcp::search::*;
use pcp::kernel::*;

pub fn solve_causal_model(session: Session, c: (Context, Vec<CausalModel>)) -> Env<Context> {
  let solver = Solver::new(session, c.0, c.1);
  solver.solve_all()
}

pub struct Solver {
  session: Session,
  context: Context,
  models: Vec<CausalModel>,
}

impl Solver {
  pub fn new(session: Session, context: Context, models: Vec<CausalModel>) -> Self {
    Solver { session, context, models }
  }

  pub fn solve_all(mut self) -> Env<Context> {
    debug!("{} causal models\n", self.models.len());
    debug!("{} instantaneous causal models\n", self.models.iter().filter(|m| m.instantaneous).count());
    for model in self.models.clone() {
      if let Some(model) = self.prepare_model(model) {
        if !self.solve_model(model) {
          break
        }
      }
    }
    if self.session.has_errors() {
      Env::fake(self.session, self.context)
    } else {
      Env::value(self.session, self.context)
    }
  }

  fn solve_model(&mut self, model: CausalModel) -> bool {
    // Search step.
    let space = model.clone().space;
    let mut search = one_solution_engine();
    search.start(&space);
    let (frozen_space, status) = search.enter(space);
    let space = frozen_space.unfreeze();

    // Print result.
    match status {
      Status::Satisfiable => {
        trace!("{:?}\n\n{:?}", space.vstore, space.cstore);
        true
      },
      Status::Unsatisfiable => {
        self.err_unsatisfiable_model();
        trace!("{:?}\n\n{:?}", space.vstore, space.cstore);
        false
      }
      Status::EndOfSearch
    | Status::Unknown(_) => unreachable!(
        "After the search step, the problem instance should be either satisfiable or unsatisfiable.")
    }
  }

  fn err_unsatisfiable_model(&self) {
    self.session.struct_span_err_with_code(DUMMY_SP,
      &format!("causality error: a write access happens after a read access on the same variable."),
      "E0033")
      .help(&"Unfortunately, we just report that the program is not causal but not the reason (see issue #4).")
      .emit();
  }

  /// Returns `None` if the model is detected unsatisfiable.
  /// In this case, an error is reported.
  fn prepare_model(&mut self, mut model: CausalModel) -> Option<CausalModel> {
    let mut unsatisfiable = false;
    let n = model.num_ops();
    for op1 in 0..n {
      for op2 in (op1+1)..n {
        if model.params.activated[op1] && model.params.activated[op2] {
          let v1 = model.params.var_of_op[op1].clone();
          let v2 = model.params.var_of_op[op2].clone();
          if v1 == v2 && model.params.is_rw_constrained(op1, op2)
          {
            debug!("{} / {} are read/write constrained.", op1, op2);
            let err_msg = "every variable access must have an explicit permission (should be done in `infer_permission.rs`).";
            let a1 = v1.permission.expect(err_msg);
            let a2 = v2.permission.expect(err_msg);
            // Enforce that every access read is done after readwrite, and in turn that every write is realized after a readwrite.
            if a1 > a2 {
              model.add_sequential_constraint(op1, op2);
            }
            else if a1 < a2 {
              model.add_sequential_constraint(op2, op1);
            }
            // Enforce that a variable is not accessed two times with readwrite.
            else if a1 == Permission::ReadWrite && a2 == Permission::ReadWrite {
              self.err_two_readwrite_accesses(v1, v2);
              unsatisfiable = true;
            }
          }
        }
      }
    }
    if unsatisfiable { None }
    else { Some(model) }
  }

  fn err_two_readwrite_accesses(&self, v1: Variable, v2: Variable) {
    self.session.struct_span_err_with_code(v2.span,
      &format!("second readwrite access to this variable."),
      "E0032")
      .span_label(v1.span, &"previous readwrite access")
      .help(&"Rational: We forbid more than one readwrite on a variable to keep the computation deterministic.\n\
            Solution 1: Remove one readwrite.\n\
            Solution 2: Encapsulate both readwrite in a single host function.\n\
            Solution 3: Separate the readwrite accesses with a delay (such as `pause`).")
      .emit();
  }
}