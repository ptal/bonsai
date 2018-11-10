// Copyright 2017 Pierre Talbot (IRCAM)

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
use back::code_formatter::*;
// use std::collections::{HashSet};

pub fn compile_statement(session: &Session, context: &Context, fmt: &mut CodeFormatter, stmt: Stmt) {
  StatementCompiler::new(session, context, fmt).compile(stmt)
}

struct StatementCompiler<'a> {
  session: &'a Session,
  context: &'a Context,
  fmt: &'a mut CodeFormatter
}

impl<'a> StatementCompiler<'a>
{
  pub fn new(session: &'a Session, context: &'a Context, fmt: &'a mut CodeFormatter) -> Self {
    StatementCompiler {
      session: session,
      context: context,
      fmt: fmt
    }
  }

  // Seq(Vec<Stmt>),
  // Par(Vec<Stmt>),
  // Space(Vec<Stmt>),
  // Let(LetStmt),
  // When(EntailmentRel, Box<Stmt>),
  // Suspend(EntailmentRel, Box<Stmt>),
  // Tell(Variable, Expr),
  // Pause,
  // PauseUp,
  // Stop,
  // Trap(Ident, Box<Stmt>),
  // Exit(Ident),
  // Loop(Box<Stmt>),
  // ProcCall(Option<Variable>, Ident),
  // ExprStmt(Expr),
  // Universe(Box<Stmt>),
  // Nothing


  fn compile(&mut self, stmt: Stmt) {
    use ast::StmtKind::*;
    match stmt.node {
      Nothing => self.nothing(),
      FnCall(java_call) => self.java_call(java_call),
      // Seq(branches) => self.sequence(branches),
      // OrPar(branches) => self.or_parallel(branches),
      // AndPar(branches) => self.and_parallel(branches),
      // Space(branches) => self.space(branches),
      // Let(body) => self.let_decl(body),
      // When(entailment, body) => self.when(entailment, body),
      // Suspend(entailment, body) => self.suspend(entailment, body),
      // Pause => self.pause(),
      // PauseUp => self.pause_up(),
      // Stop => self.stop(),
      // Trap(name, body) => self.trap(name, body),
      // Exit(name) => self.exit(name),
      // Loop(body) => self.loop_stmt(body),
      // ProcCall(process, args) => self.fun_call(process, args),
      // ModuleCall(run_expr) => self.module_call(run_expr),
      // Tell(var, expr) => self.tell(var, expr),
      // Universe(body) => self.universe(body),
      _ => ()
    }
  }

  fn nothing(&mut self) {
    self.fmt.push("new Nothing()");
  }

  fn java_call(&mut self, java_call: Expr) {
    self.fmt.push("new ClosureAtom(");
    compile_closure(self.session, self.context, self.fmt, java_call, false);
    self.fmt.push(")");
  }

  // fn nary_operator(&mut self, op_name: &str, mut branches: Vec<Stmt>)
  // {
  //   if branches.len() == 1 {
  //     self.compile(branches.pop().unwrap());
  //   }
  //   else {
  //     let mid = branches.len() / 2;
  //     let right = branches.split_off(mid);
  //     self.fmt.push_line(&format!("SC.{}(", op_name));
  //     self.fmt.indent();
  //     self.nary_operator(op_name, branches);
  //     self.fmt.terminate_line(",");
  //     self.nary_operator(op_name, right);
  //     self.fmt.push(")");
  //     self.fmt.unindent();
  //   }
  // }

  // fn sequence(&mut self, branches: Vec<Stmt>) {
  //   self.nary_operator("seq", branches);
  // }

  // fn or_parallel(&mut self, branches: Vec<Stmt>) {
  //   self.nary_operator("or_par", branches);
  // }

  // fn and_parallel(&mut self, branches: Vec<Stmt>) {
  //   self.nary_operator("and_par", branches);
  // }

  // fn space(&mut self, branches: Vec<Stmt>) {
  //   let branches_len = branches.len();
  //   self.fmt.push_line("new Space(");
  //   self.fmt.push_line("new ArrayList<>(Arrays.asList(");
  //   let uids: HashSet<String> = collect_st_vars(branches);
  //   self.fmt.indent();
  //   for uid in uids {
  //     self.fmt.push_line(&format!())
  //   }
  //   self.fmt.push_line("),");
  //   self.fmt.push_line("new ArrayList<>(Arrays.asList(");
  //   self.fmt.indent();
  //   for (i, stmt) in branches.into_iter().enumerate() {
  //     self.fmt.push_line("new SpaceBranch(");
  //     self.fmt.indent();
  //     self.compile(stmt);
  //     self.fmt.unindent();
  //     if i != branches_len - 1 {
  //       self.fmt.terminate_line("),");
  //     }
  //     else {
  //       self.fmt.push(")")
  //     }
  //   }
  //   self.fmt.unindent();
  //   self.fmt.push(")))");
  // }

  // fn let_decl(&mut self, let_decl: LetStmt) {
  //   self.fmt.push(&format!("new LocalVar("));
  //   self.binding(let_decl.binding, false, "__proc_uid.apply");
  //   self.fmt.terminate_line(",");
  //   self.compile(*let_decl.body);
  //   self.fmt.push(")");
  // }

  // fn binding(&mut self, binding: Binding, is_field: bool, uid_fn: &str)
  // {
  //   match binding.kind {
  //     Kind::Spacetime(spacetime) =>
  //       self.spacetime_binding(binding,
  //         spacetime, is_field, uid_fn),
  //     Kind::Product =>
  //       self.module_binding(binding, uid_fn),
  //     Kind::Host => panic!(
  //       "BUG: Host variables are not stored inside the \
  //        environment, and therefore binding cannot be generated.")
  //   }
  // }

  // fn spacetime_binding(&mut self,
  //   binding: Binding, spacetime: Spacetime, is_field: bool, uid_fn: &str)
  // {
  //   let spacetime = self.spacetime(spacetime);
  //   let stream_bound = context.stream_bound_of(&binding.name);
  //   self.fmt.push("new SpacetimeVar(");
  //   if is_field { self.fmt.push(&binding.name); }
  //   else { self.bottom(fmt, binding.ty.clone()); }
  //   self.fmt.push(&format!(",\"{}\", {}(\"{}\"), {}, {}, {},",
  //     binding.name, uid_fn, binding.name, spacetime,
  //     binding.is_transient(), stream_bound));
  //   self.closure(true,
  //     binding.expr.expect("BUG: Generate binding without an expression."));
  //   self.fmt.push(")");
  // }

  // fn module_binding(&mut self, binding: Binding, uid_fn: &str)
  // {
  //   self.fmt.push(&format!("new ModuleVar(\"{}\", {}(\"{}\"), ",
  //     binding.name, uid_fn, binding.name));
  //   self.closure(true,
  //     binding.expr.expect("BUG: Generate binding without an expression."));
  //   self.fmt.push(")");
  // }

  // fn spacetime(spacetime: Spacetime) -> String {
  //   use ast::Spacetime::*;
  //   match spacetime {
  //     SingleSpace(_) => String::from("Spacetime.SingleSpace"),
  //     SingleTime => String::from("Spacetime.SingleTime"),
  //     WorldLine(_) => String::from("Spacetime.WorldLine")
  //   }
  // }

  // fn entailment(&mut self, entailment: EntailmentRel) {
  //   self.fmt.push(&format!("new EntailmentConfig({}, \"", entailment.strict));
  //   self.stream_var(fmt, entailment.left.clone());
  //   self.fmt.push(&format!("\", {}, ", entailment.left.past));
  //   self.closure(true, entailment.right);
  //   self.fmt.push(")");
  // }

  // fn meta_entailment(&mut self, rel: MetaEntailmentRel) {
  //   self.fmt.push("new MetaEntailmentConfig(");
  //   self.entailment(rel.left);
  //   self.fmt.push(&format!(", {})", rel.right));
  // }

  // fn condition(&mut self, condition: Condition) {
  //   match condition {
  //     Condition::Entailment(rel) => self.entailment(rel),
  //     Condition::MetaEntailment(rel) => self.meta_entailment(rel)
  //   }
  // }

  // fn when(&mut self, condition: Condition, body: Box<Stmt>) {
  //   self.fmt.push("SC.when(");
  //   self.condition(condition);
  //   self.fmt.terminate_line(",");
  //   self.fmt.indent();
  //   self.compile(*body);
  //   self.fmt.terminate_line(",");
  //   self.fmt.push("SC.nothing())");
  //   self.fmt.unindent();
  // }

  // fn suspend(&mut self, condition: Condition, body: Box<Stmt>) {
  //   self.fmt.push("new SuspendWhen(");
  //   self.condition(condition);
  //   self.fmt.terminate_line(",");
  //   self.fmt.indent();
  //   self.compile(*body);
  //   self.fmt.push(")");
  //   self.fmt.unindent();
  // }

  // fn module_call(&mut self, run_expr: RunExpr) {
  //   self.fmt.push(&format!("new CallProcess("));
  //   let expr = run_expr.to_expr();
  //   self.closure(true, expr);
  //   self.fmt.push(")");
  // }

  // fn tell(&mut self, var: Variable, expr: Expr) {
  //   self.fmt.push("new Tell(\"");
  //   self.stream_var(fmt, var);
  //   self.fmt.push("\", ");
  //   self.closure(true, expr);
  //   self.fmt.push(")");
  // }

  // fn pause(&mut self) {
  //   self.fmt.push("SC.stop()");
  // }

  // fn pause_up(&mut self) {
  //   self.fmt.push("new PauseUp()");
  // }

  // fn stop(&mut self) {
  //   self.fmt.push("new BStop()");
  // }

  // fn loop_stmt(&mut self, body: Box<Stmt>) {
  //   self.fmt.push_line("SC.loop(");
  //   self.fmt.indent();
  //   self.compile(*body);
  //   self.fmt.unindent();
  //   self.fmt.push(")");
  // }

  // fn trap(&mut self, name: Ident, body: Box<Stmt>) {
  //   self.fmt.push_line(&format!("SC.until(\"{}\",", name));
  //   self.fmt.indent();
  //   self.compile(*body);
  //   self.fmt.unindent();
  //   self.fmt.push(")");
  // }

  // fn exit(&mut self, name: Ident) {
  //   self.fmt.push(&format!("SC.generate(\"{}\")", name));
  // }

  // fn universe(&mut self, body: Box<Stmt>) {
  //   self.fmt.push_line(&format!("new Universe({},", session.config().debug));
  //   self.fmt.indent();
  //   self.compile(*body);
  //   self.fmt.unindent();
  //   self.fmt.push(")");
  // }
}
