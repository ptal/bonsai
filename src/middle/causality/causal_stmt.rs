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

/// We capture the causal dependencies generated by statements of a spacetime program.
/// It is described in the Section 4.5.5 in the dissertation (Talbot, 2018).

use context::*;
use session::*;
use middle::causality::causal_model::*;
use middle::causality::model_parameters::*;
use middle::causality::causal_deps::*;

pub fn build_causal_model(session: Session, c: (Context, ModelParameters)) -> Env<(Context,Vec<CausalModel>)> {
  let model = CausalStmt::new(session, c.0, c.1);
  model.compute()
}

trait Continuation {
  fn call(&self, this: &CausalStmt, model: CausalModel) -> Vec<CausalModel>;
  fn bclone(&self) -> Box<Continuation>;
}

type Cont = Box<Continuation>;

#[derive(Clone)]
struct IdentityCont;

impl Continuation for IdentityCont {
  fn call(&self, _this: &CausalStmt, model: CausalModel) -> Vec<CausalModel> {
    vec![model]
  }

  fn bclone(&self) -> Cont { Box::new(self.clone()) }
}

struct SequenceCont {
  children: Vec<Stmt>,
  continuation: Cont
}

impl SequenceCont {
  pub fn new(children: Vec<Stmt>, continuation: Cont) -> Self {
    SequenceCont { children, continuation }
  }
}

impl Continuation for SequenceCont {
  fn call(&self, this: &CausalStmt, model: CausalModel) -> Vec<CausalModel> {
    this.visit_seq(self.children.clone(), model, self.continuation.bclone())
  }

  fn bclone(&self) -> Cont {
    Box::new(SequenceCont::new(self.children.clone(), self.continuation.bclone()))
  }
}

struct CausalStmt {
  session: Session,
  context: Context,
  deps: CausalDeps,
  params: ModelParameters
}

impl CausalStmt {
  pub fn new(session: Session, context: Context, params: ModelParameters) -> Self {
    let deps = CausalDeps::new();
    CausalStmt { session, context, deps, params }
  }

  fn compute(self) -> Env<(Context,Vec<CausalModel>)> {
    let bcrate_clone = self.context.clone_ast();
    let models = self.causal_analysis(bcrate_clone);
    Env::value(self.session, (self.context, models))
  }

  fn causal_analysis(&self, ast: JCrate) -> Vec<CausalModel> {
    let model = CausalModel::new(self.params.clone());
    let models = self.visit_crate(ast, model, Box::new(IdentityCont));
    models
  }

  fn visit_crate(&self, ast: JCrate, model: CausalModel,
      continuation: Cont) -> Vec<CausalModel>
  {
    let mut models = vec![];
    for module in ast.modules {
      let mut m = self.visit_module(module, model.clone(), continuation.bclone());
      models.append(&mut m);
    }
    models
  }

  fn visit_module(&self, module: JModule, model: CausalModel,
      continuation: Cont) -> Vec<CausalModel>
  {
    let mut models = vec![];
    for process in module.processes {
      // We only visit the entry points into the crate, because private processes must be called from these public processes.
      // Note: We should verify all processes, but only those that have not been called from another process.
      // Proposition: If a process "P" is not causal, then every process "C[P]" is not causal.
      // Proposition2: If a process "P" is causal, then a process "C[P]" can be not causal.
      // Consequence of proposition 2: we need to verify the causality of the libraries even if their process is causal.
      // Therefore, it is more efficient to first call the top-level process, so more analysis can be performed from here.
      if process.visibility == JVisibility::Public {
        let mut m = self.visit_process(process, model.clone(), continuation.bclone());
        models.append(&mut m);
      }
    }
    models
  }

  fn visit_process(&self, process: Process, model: CausalModel,
      continuation: Cont) -> Vec<CausalModel>
  {
    self.visit_stmt(process.body, model, continuation)
  }

  fn visit_stmt(&self, stmt: Stmt, model: CausalModel,
      continuation: Cont) -> Vec<CausalModel>
  {
    use ast::StmtKind::*;
    match stmt.node {
      Pause
    | PauseUp
    | Stop => self.visit_delay(model, continuation),
      Space(_)
    | Prune
    | Nothing => continuation.call(self, model),
      Seq(branches) => self.visit_seq(branches, model, continuation),
      Let(stmt) => self.visit_let(stmt, model, continuation),
      Tell(var, expr) => self.visit_tell(var, expr, model, continuation),
      When(cond, then_branch, else_branch) =>
        self.visit_when(cond, *then_branch, *else_branch, model, continuation),
      ExprStmt(expr) => self.visit_expr_stmt(expr, model, continuation),
      _ => panic!("not implemented")
      // Suspend(cond, body) => self.visit_suspend(cond, *body, model, continuation),
      // Abort(cond, body) => self.visit_abort(cond, *body, model, continuation),
      // OrPar(branches) => self.visit_or_par(branches),
      // AndPar(branches) => self.visit_and_par(branches),
      // Loop(body) => self.visit_loop(*body),
      // ProcCall(var, process, args) => self.visit_proc_call(var, process, args),
      // Universe(body) => self.visit_universe(*body),
    }
  }

  fn visit_delay(&self, mut model: CausalModel, _continuation: Cont) -> Vec<CausalModel>
  {
    model.instantaneous = false;
    vec![model]
  }

  fn visit_seq(&self, mut children: Vec<Stmt>,
      model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  {
    match children.len() {
      0 => continuation.call(self, model),
      1 => self.visit_stmt(children.remove(0), model, continuation),
      _ => {
        let stmt = children.remove(0);
        self.visit_stmt(stmt, model,
          Box::new(SequenceCont::new(children, continuation)))
      }
    }
  }

  fn visit_let(&self, let_stmt: LetStmt,
      model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  {
    let model = match let_stmt.binding.expr {
      None => model,
      Some(expr) => self.deps.visit_expr(expr, false, model)
    };
    self.visit_stmt(*(let_stmt.body), model, continuation)
  }

  fn visit_tell(&self, var: Variable, expr: Expr,
      model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  {
    let m1 = self.deps.visit_var(var, false, model);
    let m2 = self.deps.visit_expr(expr, false, m1);
    continuation.call(self, m2)
  }

  fn visit_when(&self, condition: Expr, then_branch: Stmt, else_branch: Stmt,
      model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  {
    let then_m = self.deps.visit_expr(condition.clone(), true, model.clone());
    let else_m = self.deps.visit_expr(condition, false, model);
    let mut m1 = self.visit_stmt(then_branch, then_m, continuation.bclone());
    let mut m2 = self.visit_stmt(else_branch, else_m, continuation);
    m1.append(&mut m2);
    m1
  }

  fn visit_expr_stmt(&self, expr: Expr,
      model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  {
    let m = self.deps.visit_expr(expr, false, model);
    continuation.call(self, m)
  }

  // fn visit_suspend(&self, condition: Expr, child: Stmt,
  //   model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  // {
  //   let then_m = self.visit_expr(condition, false, model.clone());
  //   let mut m1 = self.visit_stmt(child, then_m, continuation);
  //   let else_m = self.visit_expr(condition, true, model);
  //   else_m.instantaneous = false;
  //   m1.push(else_m);
  //   m1
  // }

  // fn visit_abort(&self, condition: Expr, child: Stmt,
  //   model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  // {
  //   let then_m = self.visit_expr(condition, false, model.clone());
  //   let else_m = self.visit_expr(condition, true, model);
  //   let mut m1 = self.visit_stmt(child, then_m, continuation.clone());
  //   let mut m2 = continuation(else_m);
  //   m1.extend(&mut m2);
  //   m1
  // }

  // fn visit_par(&self, children: Vec<Stmt>, join_termination: F,
  //   model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  //  where F: Fn(bool, bool) -> bool
  // {
  //   let mut models = vec![];
  //   for child in children {
  //     models.push(self.visit_stmt(child, model.clone(), continuation));
  //   }
  //   let first = models.remove(0);
  //   models.into_iter().fold(first, |accu, m| {
  //     let mut res = vec![];
  //     for i in 0..accu.len() {
  //       for j in 0..m.len() {
  //         res.push(accu[i].product(&m[i], join_termination));
  //       }
  //     }
  //   })
  // }

  // fn visit_or_par(&self, children: Vec<Stmt>) {
  //   self.visit_par(children)
  // }

  // fn visit_and_par(&self, children: Vec<Stmt>) {
  //   self.visit_par(children)
  // }

  // fn visit_loop(&self, child: Stmt) {
  //   self.visit_stmt(child)
  // }

  // fn visit_proc_call(&self, var: Option<Variable>, _process: Ident, args: Vec<Variable>) {
  //   walk_proc_call(self, var, args);
  // }

  // fn visit_universe(&self, child: Stmt) {
  //   self.visit_stmt(child)
  // }
}

