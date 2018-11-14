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
use back::compiler::expression::*;

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

  fn compile(&mut self, stmt: Stmt) {
    use ast::StmtKind::*;
    match stmt.node {
      Nothing => self.nothing(),
      ExprStmt(expr) => self.procedure(expr),
      QFUniverse(body) => self.qf_universe(body),
      Let(body) => self.let_decl(body),
      Seq(branches) => self.sequence(branches),
      // OrPar(branches) => self.or_parallel(branches),
      // AndPar(branches) => self.and_parallel(branches),
      // Space(branches) => self.space(branches),
      // When(entailment, body) => self.when(entailment, body),
      // Suspend(entailment, body) => self.suspend(entailment, body),
      // Pause => self.pause(),
      // PauseUp => self.pause_up(),
      // Stop => self.stop(),
      // Loop(body) => self.loop_stmt(body),
      // ProcCall(process, args) => self.fun_call(process, args),
      // ModuleCall(run_expr) => self.module_call(run_expr),
      // Tell(var, expr) => self.tell(var, expr),
      // LocalDrop(_) => (), // It is not used in the runtime.
      stmt => unimplemented!("statement unimplemented: {:?}.", stmt)
    }
  }

  fn nothing(&mut self) {
    self.fmt.push("new Nothing()");
  }

  fn procedure(&mut self, expr: Expr) {
    self.fmt.push("new ProcedureCall(");
    compile_closure(self.session, self.context, self.fmt, expr, false);
    self.fmt.push(")");
  }

  fn qf_universe(&mut self, body: Box<Stmt>) {
    self.fmt.push_line("new QFUniverse(");
    self.fmt.indent();
    self.compile(*body);
    self.fmt.unindent();
    self.fmt.push(")");
  }

  fn let_decl(&mut self, let_decl: LetStmt) {
    use ast::Kind::*;
    use ast::Spacetime::*;
    match let_decl.kind() {
      Spacetime(SingleSpace) => self.single_space_local_decl(let_decl),
      Spacetime(SingleTime) => unimplemented!("Kind::Spacetime(SingleTime) in let_decl"),
      Spacetime(WorldLine) => unimplemented!("Kind::Spacetime(WorldLine) in let_decl"),
      Product => unimplemented!("Kind::Product in let_decl"),
      Host => unimplemented!("Kind::Host in let_decl")
    }
  }

  fn single_space_local_decl(&mut self, let_decl: LetStmt) {
    self.fmt.push("new SingleSpaceVarDecl(");
    compile_local_var(self.session, self.context, self.fmt, let_decl.binding.name);
    self.fmt.push(",");
    self.fmt.indent();
    let ty = Some(let_decl.binding.ty);
    match let_decl.binding.expr.clone() {
      Some(expr) => compile_functional_expr(self.session, self.context, self.fmt, expr, ty),
      None => compile_functional_expr(self.session, self.context, self.fmt, Expr::new(DUMMY_SP, ExprKind::Bottom), ty)
    }
    self.fmt.push(",");
    self.compile(*let_decl.body);
    self.fmt.push(")");
    self.fmt.unindent();
  }

  fn nary_operator(&mut self, op_name: &str, mut branches: Vec<Stmt>)
  {
    if branches.len() == 1 {
      self.compile(branches.pop().unwrap());
    }
    else {
      self.fmt.push_line(&format!("new {}(Arrays.asList(", op_name));
      self.fmt.indent();
      let n = branches.len();
      for (i, branch) in branches.into_iter().enumerate() {
        self.compile(branch);
        if i < n - 1 {
          self.fmt.terminate_line(",");
        }
      }
      self.fmt.push("))");
      self.fmt.unindent();
    }
  }

  fn sequence(&mut self, branches: Vec<Stmt>) {
    self.nary_operator("Sequence", branches);
  }

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
}
