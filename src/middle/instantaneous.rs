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

/// We verify instantaneous constraints on various processes; an instantaneous process does not contain `pause`, `pause up` and `stop` statements.
///  * The process `b` in `space b end` must be instantaneous.
///  * The process `b` in `loop b end` must not be instantaneous.

use context::*;
use session::*;

pub fn instantaneous_analysis(session: Session, context: Context) -> Env<Context> {
  let analysis = InstantaneousAnalysis::new(session, context);
  analysis.analyse()
}

struct InstantaneousAnalysis {
  session: Session,
  context: Context,
  can_pause: bool,
  must_pause: bool,
  context_span: Span,
  current_module: Ident
}

impl InstantaneousAnalysis {
  pub fn new(session: Session, context: Context) -> Self {
    let dummy_ident = context.dummy_ident();
    InstantaneousAnalysis {
      session: session,
      context: context,
      can_pause: false,
      must_pause: false,
      context_span: DUMMY_SP,
      current_module: dummy_ident,
    }
  }

  fn analyse(mut self) -> Env<Context> {
    let bcrate_clone = self.context.clone_ast();
    self.visit_crate(bcrate_clone);
    if self.session.has_errors() {
      Env::fake(self.session, self.context)
    } else {
      Env::value(self.session, self.context)
    }
  }

  fn err_instantaneous_loop(&self) {
    self.session.struct_span_err_with_code(self.context_span,
      &format!("forbidden loop with an instantaneous body."),
      "E0028")
    .help(&"Every path of the body of a `loop` statement must at least contain one delay statement (`pause`, `pause up` and `stop`).\n\
           This error easily occurs with a `when` statement without an `else` branch, or `abort` within the loop which can be instantaneous.")
    .emit();
  }

  fn err_non_instantaneous_space(&self) {
    self.session.struct_span_err_with_code(self.context_span,
      &format!("process not instantaneous in a `space` statement."),
      "E0029")
    .help(&"Process in `space` statement must be instantaneous, that is it must not contain delay (`pause`, `pause up` and `stop`) or `suspend` statements.")
    .emit();
  }

  fn visit_stmts(&mut self, stmts: Vec<Stmt>) -> (Vec<bool>, Vec<bool>) {
    let mut can = vec![];
    let mut must = vec![];
    for stmt in stmts {
      self.visit_stmt(stmt);
      can.push(self.can_pause);
      must.push(self.must_pause);
    }
    (can, must)
  }
}

impl Visitor<JClass> for InstantaneousAnalysis
{
  fn visit_module(&mut self, module: JModule) {
    let old = self.current_module.clone();
    self.current_module = module.mod_name();
    walk_processes(self, module.processes);
    self.current_module = old;
  }

  fn visit_stmt(&mut self, child: Stmt) {
    let old = self.context_span;
    self.context_span = child.span;
    self.can_pause = false;
    self.must_pause = false;
    walk_stmt(self, child);
    self.context_span = old;
  }

  fn visit_when(&mut self, _condition: Expr, then_branch: Stmt, else_branch: Stmt) {
    self.visit_stmt(then_branch);
    let then_must = self.must_pause;
    let then_can = self.can_pause;
    self.visit_stmt(else_branch);
    self.must_pause = self.must_pause && then_must;
    self.can_pause = self.can_pause || then_can;
  }

  fn visit_delay(&mut self, _delay: Delay) {
    self.can_pause = true;
    self.must_pause = true;
  }

  fn visit_loop(&mut self, child: Stmt) {
    self.visit_stmt(child);
    if !self.must_pause {
      self.err_instantaneous_loop()
    }
  }

  fn visit_space(&mut self, child: Stmt) {
    self.visit_stmt(child);
    if self.can_pause {
      self.err_non_instantaneous_space();
    }
    self.can_pause = false;
    self.must_pause = false;
  }

  fn visit_seq(&mut self, children: Vec<Stmt>) {
    let (can, must) = self.visit_stmts(children);
    for i in 0..can.len() {
      self.can_pause = self.can_pause || can[i];
      self.must_pause = self.must_pause || must[i];
    }
  }

  fn visit_par(&mut self, children: Vec<Stmt>) {
    let (can, must) = self.visit_stmts(children);
    for i in 0..can.len() {
      self.can_pause = self.can_pause || can[i];
      // If one process must pause, then they all pause.
      self.must_pause = self.must_pause || must[i];
    }
  }

  fn visit_suspend(&mut self, suspend: SuspendStmt) {
    self.visit_stmt(*suspend.body);
    self.can_pause = self.can_pause || true;
  }

  fn visit_abort(&mut self, _condition: Expr, child: Stmt) {
    self.visit_stmt(child);
    self.must_pause = false;
  }

  fn visit_proc_call(&mut self, var: Option<Variable>, process: Ident, _args: Vec<Variable>) {
    let (uid, process) = self.context.find_proc_from_call(self.current_module.clone(), process, var);
    let old = self.current_module.clone();
    self.current_module = uid.module;
    self.visit_process(process);
    self.current_module = old;
  }
}
