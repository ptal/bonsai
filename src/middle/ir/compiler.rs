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

use context::*;
use session::*;
use middle::causality::symbolic_execution::State;
use middle::ir::guarded_command::*;
use middle::ir::scheduling::*;
use std::collections::HashMap;


trait Continuation {
  fn call(&self, this: &Compiler, guard: Box<Expr>) -> Vec<GuardedProgram>;
  fn bclone(&self) -> Box<Continuation>;
}

type Cont = Box<Continuation>;

#[derive(Clone)]
struct IdentityCont;

impl Continuation for IdentityCont {
  fn call(&self, _this: &Compiler, guard: Box<Expr>) -> Vec<GuardedProgram> {
    // vec![model]
    vec![]
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
  fn call(&self, this: &Compiler, guard: Box<Expr>) -> Vec<GuardedProgram> {
    // this.visit_seq(self.children.clone(), model, self.continuation.bclone())
    vec![]
  }

  fn bclone(&self) -> Cont {
    Box::new(SequenceCont::new(self.children.clone(), self.continuation.bclone()))
  }
}



pub type AllInstants = HashMap<ProcessUID, Vec<Instant>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instant {
  pub locations: State,
  pub program: Stmt,
  pub path_schedules: Vec<Scheduling>,
}

impl Instant
{
  pub fn init(locations: State, program: Stmt) -> Self {
    Self::new(locations, program, vec![])
  }

  pub fn new(locations: State, program: Stmt, path_schedules: Vec<Scheduling>) -> Self {
    Instant { locations, program, path_schedules }
  }
}

pub struct Compiler {
  session: Session,
  context: Context,
  factory: GuardedCommandFactory,
  ir: IR,
}

impl Compiler {
  pub fn new(session: Session, context: Context, instants: &AllInstants) -> Self {
    let factory = GuardedCommandFactory::new(&instants);
    Compiler {
      session, context,
      ir: IR::new(),
      factory
    }
  }

  pub fn compile(mut self, instants: AllInstants) -> Env<(Context, IR)> {
    for instants_per_process in instants {
      self.compile_process_instants(instants_per_process);
    }
    Env::value(self.session, (self.context, self.ir))
  }

  fn compile_process_instants(&mut self, (process, instants): (ProcessUID, Vec<Instant>)) {
    for instant in instants {
      let execution_paths = self.compile_instant(instant.locations, instant.program);
      let program = self.schedule_paths(execution_paths, instant.path_schedules);
      self.ir.processes.insert(process.clone(), program);
    }
  }

  fn compile_instant(&mut self, locations: State, program: Stmt) -> Vec<GuardedProgram> {
    let guard = self.factory.create_locations_guard(locations);
    self.compile_stmt(program, guard, Box::new(IdentityCont))
  }

  fn compile_stmt(&self, program: Stmt, guard: Box<Expr>,
    continuation: Cont) -> Vec<GuardedProgram>
  {
    // use ast::StmtKind::*;
    match program.node {
      // DelayStmt(delay) => self.compile_delay(delay, guard, continuation),
      // Space(p) => self.compile_space(p, guard, continuation),
      // Prune => self.compile_prune(guard, continuation),
      // LocalDrop(binding) => self.compile_local_drop(binding, guard, continuation),
      // Nothing => continuation.call(self, guard),
      // Seq(branches) => self.compile_seq(branches, guard, continuation),
      // Let(stmt) => self.compile_let(stmt, guard, continuation),
      // Tell(var, expr) => self.compile_tell(var, expr, guard, continuation),
      // When(cond, then_branch, else_branch) =>
      //   self.compile_when(cond, *then_branch, *else_branch, guard, continuation),
      // ExprStmt(expr) => self.compile_expr_stmt(expr, guard, continuation),
      // OrPar(branches) => self.compile_or_par(branches, guard, continuation),
      // AndPar(branches) => self.compile_and_par(branches, guard, continuation),
      _ => vec![]
      // Suspend(cond, body) => self.compile_suspend(cond, *body, guard, continuation),
      // Abort(cond, body) => self.compile_abort(cond, *body, guard, continuation),
      // Loop(body) => self.compile_loop(*body),
      // ProcCall(var, process, args) => self.compile_proc_call(var, process, args),
      // Universe(body) => self.compile_universe(*body),
    }
  }

  // fn compile_delay(&self, delay: Delay, guard: Box<Expr>,
  //   _continuation: Cont) -> Vec<GuardedProgram>
  // {
  //   let delay_command = self.factory.create_delay(delay, guard);
  //   vec![vec![delay_command]]
  // }

  // fn compile_seq(&self, mut children: Vec<Stmt>,
  //     model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  // {
  //   match children.len() {
  //     0 => continuation.call(self, model),
  //     1 => self.compile_stmt(children.remove(0), model, continuation),
  //     _ => {
  //       let stmt = children.remove(0);
  //       self.compile_stmt(stmt, model,
  //         Box::new(SequenceCont::new(children, continuation)))
  //     }
  //   }
  // }

  // fn compile_let(&self, let_stmt: LetStmt,
  //     model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  // {
  //   let model = match let_stmt.binding.expr {
  //     None => model,
  //     Some(expr) => self.deps.compile_expr(expr, None, model)
  //   };
  //   self.compile_stmt(*(let_stmt.body), model, continuation)
  // }

  // fn compile_tell(&self, var: Variable, expr: Expr,
  //     model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  // {
  //   let m1 = self.deps.compile_expr(expr, None, model);
  //   let m2 = self.deps.compile_var(var, None, m1);
  //   continuation.call(self, m2)
  // }

  // fn compile_when(&self, condition: Expr, then_branch: Stmt, else_branch: Stmt,
  //     model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  // {
  //   let then_m = self.deps.compile_expr(condition.clone(), Some(true), model.clone());
  //   let mut m1 = self.compile_stmt(then_branch, then_m, continuation.bclone());
  //   let else_m = self.deps.compile_expr(condition, Some(false), model);
  //   let mut m2 = self.compile_stmt(else_branch, else_m, continuation);
  //   m1.append(&mut m2);
  //   m1
  // }

  // fn compile_expr_stmt(&self, expr: Expr,
  //     model: CausalModel, continuation: Cont) -> Vec<CausalModel>
  // {
  //   let m = self.deps.compile_expr(expr, None, model);
  //   continuation.call(self, m)
  // }

  // fn compile_par<F>(&self, children: Vec<Stmt>, model: CausalModel,
  //   continuation: Cont, join_termination: F) -> Vec<CausalModel>
  //  where F: Clone + Fn(bool, bool) -> bool
  // {
  //   // We create the model of every branch without calling the current continuation.
  //   let mut models = vec![];
  //   for child in children {
  //     models.push(self.compile_stmt(child, model.clone(), Box::new(IdentityCont)));
  //   }
  //   // We merge by Cartesian product the models created by every branch.
  //   let first = models.remove(0);
  //   let models = models.into_iter().fold(first, |accu, m| {
  //     CausalModel::cartesian_product(accu, m, join_termination.clone())
  //   });
  //   // We call the continuation on models that are instantaneous.
  //   let mut result = vec![];
  //   for model in models {
  //     if model.instantaneous {
  //       let mut next = continuation.call(self, model);
  //       result.append(&mut next);
  //     }
  //     else {
  //       result.push(model);
  //     }
  //   }
  //   result
  // }

  // fn compile_or_par(&self, children: Vec<Stmt>, model: CausalModel,
  //   continuation: Cont) -> Vec<CausalModel>
  // {
  //   self.compile_par(children, model, continuation, CausalModel::term_or)
  // }

  // fn compile_and_par(&self, children: Vec<Stmt>, model: CausalModel,
  //   continuation: Cont) -> Vec<CausalModel>
  // {
  //   self.compile_par(children, model, continuation, CausalModel::term_and)
  // }


  fn schedule_paths(&mut self, execution_paths: Vec<GuardedProgram>,
    schedules: Vec<Scheduling>) -> GuardedProgram
  {
    let mut program = GuardedProgram::new();
    for (path, schedule) in execution_paths.into_iter().zip(schedules.into_iter()) {
      let mut path = self.schedule_path(path, schedule);
      program.append(&mut path);
    }
    program
  }

  fn schedule_path(&mut self, path: GuardedProgram, schedule: Scheduling) -> GuardedProgram {
    GuardedProgram::new()
  }
}
