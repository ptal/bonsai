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

/// Given a progam, `Indexing` has two tasks:
///   1. Indexing every access operation with an integer.
///      For example, `read x` becomes `read^n x` where `n` is its index (field `op_no` in `Variable`).
///      We also create a `reversed index lookup` in `ModelParameters` where we can search a variable from an operation number.
///   2. Indexing every pause-like statements (`pause`,`pause up`,`stop`,`suspend`) with an integer.
///      This is useful to represent an instant with a compact state (instead of the full AST).
///      The first n indexes are reserved to the entry points of the program.

use context::*;
use session::*;
use middle::causality::model_parameters::*;

/// Returns the modified AST with indexed variable operations and the associated model parameters.
pub fn index_ops_and_delay(session: Session, context: Context) -> Env<(Context, ModelParameters)> {
  let index = Indexing::new(session, context);
  index.compute()
}

struct Indexing {
  session: Session,
  context: Context,
  params: ModelParameters,
  state_num: usize
}

impl Indexing {
  pub fn new(session: Session, context: Context) -> Self {
    let entry_points_indexes = context.entry_points.len();
    Indexing {
      session, context,
      params: ModelParameters::new(),
      state_num: entry_points_indexes
    }
  }

  fn compute(mut self) -> Env<(Context, ModelParameters)> {
    let mut bcrate_clone = self.context.clone_ast();
    self.visit_crate(&mut bcrate_clone);
    self.context.replace_ast(bcrate_clone);
    Env::value(self.session, (self.context, self.params))
  }

  fn gen_state(&mut self) -> usize {
    self.state_num += 1;
    self.state_num
  }
}

impl VisitorMut<JClass> for Indexing
{
  fn visit_delay(&mut self, delay: &mut Delay) {
    delay.state_num = self.gen_state();
  }

  fn visit_suspend(&mut self, suspend: &mut SuspendStmt) {
    suspend.state_num = self.gen_state();
    walk_suspend_mut(self, suspend)
  }

  fn visit_var(&mut self, var: &mut Variable) {
    self.params.alloc_variable(var);
  }
}
