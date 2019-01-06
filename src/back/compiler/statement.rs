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
use back::free_variables::*;
use back::compiler::expression::*;
use trilean::SKleene;

pub fn compile_statement(session: &Session, context: &Context, fmt: &mut CodeFormatter, mod_name: Ident, stmt: Stmt) {
  StatementCompiler::new(session, context, mod_name, fmt).compile(stmt)
}

struct StatementCompiler<'a> {
  session: &'a Session,
  context: &'a Context,
  mod_name: Ident,
  fmt: &'a mut CodeFormatter
}

impl<'a> StatementCompiler<'a>
{
  pub fn new(session: &'a Session, context: &'a Context, mod_name: Ident, fmt: &'a mut CodeFormatter) -> Self {
    StatementCompiler {
      session, context, mod_name, fmt
    }
  }

  fn compile(&mut self, stmt: Stmt) {
    use ast::StmtKind::*;
    match stmt.node {
      Nothing => self.nothing(),
      DelayStmt(delay) => self.delay(delay),
      Space(branch) => self.space(branch),
      Prune => self.prune(),
      ExprStmt(expr) => self.procedure(expr),
      Let(body) => self.let_decl(body),
      Seq(branches) => self.sequence(branches),
      When(cond, then, els) => self.when(cond, then, els),
      QFUniverse(body) => self.qf_universe(body),
      Universe(queue, body) => self.universe(queue, body),
      Tell(var, expr) => self.tell(var, expr),
      OrPar(branches) => self.or_parallel(branches),
      AndPar(branches) => self.and_parallel(branches),
      Loop(body) => self.loop_stmt(body),
      // Suspend(entailment, body) => self.suspend(entailment, body),
      // ProcCall(process, args) => self.fun_call(process, args),
      // ModuleCall(run_expr) => self.module_call(run_expr),
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

  fn let_decl(&mut self, let_decl: LetStmt) {
    use ast::Kind::*;
    use ast::Spacetime::*;
    match let_decl.kind() {
      Spacetime(SingleSpace) => self.single_space_local_decl(let_decl),
      Spacetime(SingleTime) => self.single_time_local_decl(let_decl),
      Spacetime(WorldLine) => self.world_line_local_decl(let_decl),
      Product => unimplemented!("Kind::Product in let_decl"),
      Host => unimplemented!("Kind::Host in let_decl")
    }
  }

  fn local_decl(&mut self, let_decl: LetStmt, decl_class: &str) {
    self.fmt.push(&format!("new {}(", decl_class));
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

  fn single_space_local_decl(&mut self, let_decl: LetStmt) {
    self.local_decl(let_decl, "SingleSpaceVarDecl");
  }

  fn single_time_local_decl(&mut self, let_decl: LetStmt) {
    self.local_decl(let_decl, "SingleTimeVarDecl");
  }

  fn world_line_local_decl(&mut self, let_decl: LetStmt) {
    self.local_decl(let_decl, "WorldLineVarDecl");
  }

  fn nary_operator(&mut self, op_name: &str, mut branches: Vec<Stmt>, extra: Option<&str>)
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
      self.fmt.push(")");
      if let Some(e) = extra {
        self.fmt.push(&format!(", {}", e));
      }
      self.fmt.push(")");
      self.fmt.unindent();
    }
  }

  fn sequence(&mut self, branches: Vec<Stmt>) {
    self.nary_operator("Sequence", branches, None);
  }

  fn delay(&mut self, delay: Delay) {
    self.fmt.push("new Delay(");
    match delay.kind {
      DelayKind::Pause => self.fmt.push("CompletionCode.PAUSE"),
      DelayKind::PauseUp => self.fmt.push("CompletionCode.PAUSE_UP"),
      DelayKind::Stop => self.fmt.push("CompletionCode.STOP"),
    }
    self.fmt.push(")");
  }

  fn space(&mut self, branch: Box<Stmt>) {
    let free_vars = free_variables(self.context, self.mod_name.clone(), (*branch).clone());
    self.fmt.push_line("new SpaceStmt(");
    self.fmt.indent();
    self.fmt.push_line("new ArrayList<>(Arrays.asList(");
    self.fmt.indent();
    let n = free_vars.len();
    for (i, var) in free_vars.into_iter().enumerate() {
      generate_var_uid(self.session, self.context, self.fmt, var);
      if i != n - 1 {
        self.fmt.push(", ")
      }
    }
    self.fmt.terminate_line(")),");
    self.fmt.unindent();
    self.compile(*branch);
    self.fmt.push(")");
  }

  fn prune(&mut self) {
    self.fmt.push("new Prune()");
  }

  fn qf_universe(&mut self, body: Box<Stmt>) {
    self.fmt.push_line("new QFUniverse(");
    self.fmt.indent();
    self.compile(*body);
    self.fmt.unindent();
    self.fmt.push(")");
  }

  fn universe(&mut self, queue: Variable, body: Box<Stmt>) {
    self.fmt.push_line("new Universe(");
    self.fmt.indent();
    generate_var_uid(self.session, self.context, self.fmt, queue);
    self.fmt.terminate_line(", ");
    self.compile(*body);
    self.fmt.unindent();
    self.fmt.push(")");
  }

  fn condition(&mut self, mut cond: Expr) {
    let rel = match cond.node.clone() {
      ExprKind::Entailment(rel) => rel,
      _ =>
        // transform x to x |= true
        // TODO: should be x == true.
        Box::new(EntailmentRel {
          left: cond.clone(),
          right: Expr::new(DUMMY_SP, ExprKind::Trilean(SKleene::True)),
          strict: false
        })
    };
    cond.node = ExprKind::Entailment(rel);
    compile_functional_expr(self.session, self.context, self.fmt, cond, None);
  }

  fn when(&mut self, cond: Expr, then: Box<Stmt>, els: Box<Stmt>) {
    self.fmt.push("new WhenElse(");
    self.fmt.indent();
    self.condition(cond);
    self.fmt.terminate_line(",");
    self.compile(*then);
    self.fmt.terminate_line(",");
    self.compile(*els);
    self.fmt.push(")");
    self.fmt.unindent();
  }

  // We rewrite `x <- v` to `x.join_in_place(v)`.
  fn tell(&mut self, var: Variable, expr: Expr) {
    let span = expr.span.clone();
    let node = ExprKind::Call(MethodCall::new(
      span.clone(), Some(var), Ident::gen("join_in_place"), vec![expr]));
    self.procedure(Expr::new(span, node));
  }


  fn or_parallel(&mut self, branches: Vec<Stmt>) {
    self.nary_operator("LayeredParallel", branches,
      Some("LayeredParallel.CONJUNCTIVE_PAR"));
  }

  fn and_parallel(&mut self, branches: Vec<Stmt>) {
    self.nary_operator("LayeredParallel", branches,
      Some("LayeredParallel.DISJUNCTIVE_PAR"));
  }

  fn loop_stmt(&mut self, body: Box<Stmt>) {
    self.fmt.push_line("new Loop(");
    self.fmt.indent();
    self.compile(*body);
    self.fmt.push(")");
    self.fmt.unindent();
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

  // fn module_binding(&mut self, binding: Binding, uid_fn: &str)
  // {
  //   self.fmt.push(&format!("new ModuleVar(\"{}\", {}(\"{}\"), ",
  //     binding.name, uid_fn, binding.name));
  //   self.closure(true,
  //     binding.expr.expect("BUG: Generate binding without an expression."));
  //   self.fmt.push(")");
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
}
