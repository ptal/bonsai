// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ast::*;
use ast::StmtKind::*;
use ast::ExprKind::*;

pub trait Visitor<H>
{
  fn visit_crate(&mut self, bcrate: Crate<H>) {
    walk_modules(self, bcrate.modules);
  }

  fn visit_module(&mut self, module: Module<H>) {
    walk_fields(self, module.fields);
    walk_processes(self, module.processes);
  }

  fn visit_field(&mut self, field: ModuleField) {
    self.visit_binding(field.binding)
  }

  fn visit_process(&mut self, process: Process) {
    self.visit_stmt(process.body)
  }

  fn visit_stmt(&mut self, child: Stmt) {
    walk_stmt(self, child)
  }

  fn visit_seq(&mut self, children: Vec<Stmt>) {
    walk_stmts(self, children);
  }

  fn visit_par(&mut self, children: Vec<Stmt>) {
    walk_stmts(self, children);
  }

  fn visit_or_par(&mut self, children: Vec<Stmt>) {
    self.visit_par(children)
  }

  fn visit_and_par(&mut self, children: Vec<Stmt>) {
    self.visit_par(children)
  }

  fn visit_space(&mut self, child: Stmt) {
    self.visit_stmt(child);
  }

  fn visit_prune(&mut self) {
  }

  fn visit_let(&mut self, let_stmt: LetStmt) {
    self.visit_binding(let_stmt.binding);
    self.visit_stmt(*(let_stmt.body))
  }

  fn visit_when(&mut self, condition: Expr, then_branch: Stmt, else_branch: Stmt) {
    self.visit_expr(condition);
    self.visit_stmt(then_branch);
    self.visit_stmt(else_branch);
  }

  fn visit_suspend(&mut self, suspend: SuspendStmt) {
    walk_suspend(self, suspend)
  }

  fn visit_abort(&mut self, condition: Expr, child: Stmt) {
    self.visit_expr(condition);
    self.visit_stmt(child)
  }

  fn visit_tell(&mut self, var: Variable, expr: Expr) {
    self.visit_var(var);
    self.visit_expr(expr);
  }

  fn visit_delay(&mut self, _delay: Delay) {}

  fn visit_loop(&mut self, child: Stmt) {
    self.visit_stmt(child)
  }

  fn visit_proc_call(&mut self, var: Option<Variable>, _process: Ident, args: Vec<Variable>) {
    walk_proc_call(self, var, args);
  }

  fn visit_expr_stmt(&mut self, expr: Expr) {
    self.visit_expr(expr);
  }

  fn visit_nothing(&mut self) {}

  fn visit_qf_universe(&mut self, child: Stmt) {
    self.visit_stmt(child)
  }

  fn visit_universe(&mut self, queue: Variable, child: Stmt) {
    self.visit_var(queue);
    self.visit_stmt(child)
  }

  fn visit_binding(&mut self, binding: Binding) {
    walk_binding(self, binding)
  }

  fn visit_expr(&mut self, expr: Expr) {
    walk_expr(self, expr)
  }

  fn visit_entailment(&mut self, rel: EntailmentRel) {
    self.visit_expr(rel.left);
    self.visit_expr(rel.right);
  }

  fn visit_new_instance(&mut self, _ty: JType, args: Vec<Expr>) {
    walk_exprs(self, args)
  }

  fn visit_method_call(&mut self, call: MethodCall) {
    walk_method_call(self, call)
  }

  fn visit_number(&mut self, _value: i64) {}
  fn visit_string_lit(&mut self, _value: String) {}
  fn visit_var(&mut self, _var: Variable) {}
  fn visit_bot(&mut self) {}
  fn visit_top(&mut self) {}

  fn visit_trilean(&mut self, _value: SKleene) {}

  fn visit_trilean_or(&mut self, left: Expr, right: Expr) {
    self.visit_expr(left);
    self.visit_expr(right);
  }

  fn visit_trilean_and(&mut self, left: Expr, right: Expr) {
    self.visit_expr(left);
    self.visit_expr(right);
  }

  fn visit_trilean_not(&mut self, expr: Expr) {
    self.visit_expr(expr);
  }
}

pub fn walk_modules<H, V: ?Sized>(visitor: &mut V, modules: Vec<Module<H>>) where
  V: Visitor<H>
{
  for module in modules {
    visitor.visit_module(module);
  }
}

pub fn walk_fields<H, V: ?Sized>(visitor: &mut V, fields: Vec<ModuleField>) where
  V: Visitor<H>
{
  for field in fields {
    visitor.visit_field(field);
  }
}

pub fn walk_processes<H, V: ?Sized>(visitor: &mut V, processes: Vec<Process>) where
  V: Visitor<H>
{
  for process in processes {
    visitor.visit_process(process);
  }
}

pub fn walk_stmt<H, V: ?Sized>(visitor: &mut V, stmt: Stmt) where
  V: Visitor<H>
{
  match stmt.node {
    Seq(branches) => visitor.visit_seq(branches),
    OrPar(branches) => visitor.visit_or_par(branches),
    AndPar(branches) => visitor.visit_and_par(branches),
    Space(branch) => visitor.visit_space(*branch),
    Prune => visitor.visit_prune(),
    Let(stmt) => visitor.visit_let(stmt),
    When(cond, then_branch, else_branch) => visitor.visit_when(cond, *then_branch, *else_branch),
    Suspend(suspend) => visitor.visit_suspend(suspend),
    Abort(cond, body) => visitor.visit_abort(cond, *body),
    Tell(var, expr) => visitor.visit_tell(var, expr),
    DelayStmt(delay) => visitor.visit_delay(delay),
    Loop(body) => visitor.visit_loop(*body),
    ExprStmt(expr) => visitor.visit_expr_stmt(expr),
    ProcCall(var, process, args) => visitor.visit_proc_call(var, process, args),
    QFUniverse(body) => visitor.visit_qf_universe(*body),
    Universe(queue, body) => visitor.visit_universe(queue, *body),
    Nothing => visitor.visit_nothing(),
  }
}

pub fn walk_stmts<H, V: ?Sized>(visitor: &mut V, stmts: Vec<Stmt>) where
  V: Visitor<H>
{
  for stmt in stmts {
    visitor.visit_stmt(stmt);
  }
}

pub fn walk_expr<H, V: ?Sized>(visitor: &mut V, expr: Expr) where
  V: Visitor<H>
{
  match expr.node {
    Number(value) => visitor.visit_number(value),
    StringLiteral(value) => visitor.visit_string_lit(value),
    NewInstance(new_instance) => visitor.visit_new_instance(new_instance.ty, new_instance.args),
    Call(call) => visitor.visit_method_call(call),
    // Bonsai expressions
    Var(var) => visitor.visit_var(var),
    Bottom => visitor.visit_bot(),
    Top => visitor.visit_top(),
    // Trilean
    Trilean(value) => visitor.visit_trilean(value),
    Or(left, right) => visitor.visit_trilean_or(*left, *right),
    And(left, right) => visitor.visit_trilean_and(*left, *right),
    Not(expr) => visitor.visit_trilean_not(*expr),
    Entailment(rel) => visitor.visit_entailment(*rel)
  }
}

pub fn walk_exprs<H, V: ?Sized>(visitor: &mut V, exprs: Vec<Expr>) where
  V: Visitor<H>
{
  for expr in exprs {
    visitor.visit_expr(expr);
  }
}

pub fn walk_method_call<H, V: ?Sized>(visitor: &mut V, call: MethodCall) where
  V: Visitor<H>
{
  if let Some(target) = call.target {
    visitor.visit_var(target);
  }
  walk_exprs(visitor, call.args)
}

pub fn walk_binding<H, V: ?Sized>(visitor: &mut V, binding: Binding) where
  V: Visitor<H>
{
  if let Some(expr) = binding.expr { visitor.visit_expr(expr) }
}

pub fn walk_proc_call<H, V: ?Sized>(visitor: &mut V, var: Option<Variable>, args: Vec<Variable>) where
  V: Visitor<H>
{
  if let Some(var) = var { visitor.visit_var(var) }
  for arg in args {
    visitor.visit_var(arg);
  }
}

pub fn walk_suspend<H, V: ?Sized>(visitor: &mut V, suspend: SuspendStmt) where
  V: Visitor<H>
{
  visitor.visit_expr(suspend.condition);
  visitor.visit_stmt(*suspend.body)
}

pub trait VisitorMut<H>
{
  fn visit_crate(&mut self, bcrate: &mut Crate<H>) {
    walk_modules_mut(self, &mut bcrate.modules);
  }

  fn visit_module(&mut self, module: &mut Module<H>) {
    walk_fields_mut(self, &mut module.fields);
    walk_processes_mut(self, &mut module.processes);
  }

  fn visit_field(&mut self, field: &mut ModuleField) {
    self.visit_binding(&mut field.binding)
  }

  fn visit_process(&mut self, process: &mut Process) {
    self.visit_stmt(&mut process.body)
  }

  fn visit_stmt(&mut self, child: &mut Stmt) {
    walk_stmt_mut(self, child)
  }

  fn visit_seq(&mut self, children: &mut Vec<Stmt>) {
    walk_stmts_mut(self, children);
  }

  fn visit_par(&mut self, children: &mut Vec<Stmt>) {
    walk_stmts_mut(self, children);
  }

  fn visit_or_par(&mut self, children: &mut Vec<Stmt>) {
    self.visit_par(children)
  }

  fn visit_and_par(&mut self, children: &mut Vec<Stmt>) {
    self.visit_par(children);
  }

  fn visit_space(&mut self, child: &mut Stmt) {
    self.visit_stmt(child);
  }

  fn visit_prune(&mut self) {
  }

  fn visit_let(&mut self, let_stmt: &mut LetStmt) {
    self.visit_binding(&mut let_stmt.binding);
    self.visit_stmt(&mut *(let_stmt.body))
  }

  fn visit_when(&mut self, condition: &mut Expr, then_branch: &mut Stmt, else_branch: &mut Stmt) {
    self.visit_expr(condition);
    self.visit_stmt(then_branch);
    self.visit_stmt(else_branch);
  }

  fn visit_suspend(&mut self, suspend: &mut SuspendStmt) {
    walk_suspend_mut(self, suspend)
  }

  fn visit_abort(&mut self, condition: &mut Expr, child: &mut Stmt) {
    self.visit_expr(condition);
    self.visit_stmt(child)
  }

  fn visit_tell(&mut self, var: &mut Variable, expr: &mut Expr) {
    self.visit_var(var);
    self.visit_expr(expr);
  }

  fn visit_delay(&mut self, _delay: &mut Delay) {}

  fn visit_loop(&mut self, child: &mut Stmt) {
    self.visit_stmt(child)
  }

  fn visit_proc_call(&mut self, var: &mut Option<Variable>, _process: Ident, args: &mut Vec<Variable>) {
    walk_proc_call_mut(self, var, args)
  }

  fn visit_expr_stmt(&mut self, expr: &mut Expr) {
    self.visit_expr(expr)
  }

  fn visit_nothing(&mut self) {}

  fn visit_qf_universe(&mut self, child: &mut Stmt) {
    self.visit_stmt(child)
  }

  fn visit_universe(&mut self, queue: &mut Variable, child: &mut Stmt) {
    self.visit_var(queue);
    self.visit_stmt(child)
  }

  fn visit_binding(&mut self, binding: &mut Binding) {
    walk_binding_mut(self, binding);
  }

  fn visit_expr(&mut self, expr: &mut Expr) {
    walk_expr_mut(self, expr)
  }

  fn visit_new_instance(&mut self, _ty: JType, args: &mut Vec<Expr>) {
    walk_exprs_mut(self, args)
  }

  fn visit_method_call(&mut self, call: &mut MethodCall) {
    walk_method_call_mut(self, call)
  }

  fn visit_entailment(&mut self, rel: &mut EntailmentRel) {
    self.visit_expr(&mut rel.left);
    self.visit_expr(&mut rel.right);
  }

  fn visit_var(&mut self, _var: &mut Variable) {}
  fn visit_number(&mut self, _value: i64) {}
  fn visit_string_lit(&mut self, _value: String) {}
  fn visit_bot(&mut self) {}
  fn visit_top(&mut self) {}

  fn visit_trilean(&mut self, _value: SKleene) {}

  fn visit_trilean_or(&mut self, left: &mut Expr, right: &mut Expr) {
    self.visit_expr(left);
    self.visit_expr(right);
  }

  fn visit_trilean_and(&mut self, left: &mut Expr, right: &mut Expr) {
    self.visit_expr(left);
    self.visit_expr(right);
  }

  fn visit_trilean_not(&mut self, expr: &mut Expr) {
    self.visit_expr(expr);
  }
}

pub fn walk_modules_mut<H, V: ?Sized>(visitor: &mut V, modules: &mut Vec<Module<H>>) where
  V: VisitorMut<H>
{
  for module in modules {
    visitor.visit_module(module);
  }
}

pub fn walk_fields_mut<H, V: ?Sized>(visitor: &mut V, fields: &mut Vec<ModuleField>) where
  V: VisitorMut<H>
{
  for field in fields {
    visitor.visit_field(field);
  }
}

pub fn walk_processes_mut<H, V: ?Sized>(visitor: &mut V, processes: &mut Vec<Process>) where
  V: VisitorMut<H>
{
  for process in processes {
    visitor.visit_process(process);
  }
}

pub fn walk_stmt_mut<H, V: ?Sized>(visitor: &mut V, stmt: &mut Stmt) where
  V: VisitorMut<H>
{
  match &mut stmt.node {
    &mut Seq(ref mut branches) => visitor.visit_seq(branches),
    &mut OrPar(ref mut branches) => visitor.visit_or_par(branches),
    &mut AndPar(ref mut branches) => visitor.visit_and_par(branches),
    &mut Space(ref mut branch) => visitor.visit_space(&mut **branch),
    &mut Prune => visitor.visit_prune(),
    &mut Let(ref mut stmt) => visitor.visit_let(stmt),
    &mut When(ref mut cond, ref mut then_branch, ref mut else_branch) => visitor.visit_when(cond, &mut **then_branch, &mut **else_branch),
    &mut Suspend(ref mut suspend) => visitor.visit_suspend(suspend),
    &mut Abort(ref mut cond, ref mut body) => visitor.visit_abort(cond, &mut **body),
    &mut Tell(ref mut var, ref mut expr) => visitor.visit_tell(var, expr),
    &mut DelayStmt(ref mut delay) => visitor.visit_delay(delay),
    &mut Loop(ref mut body) => visitor.visit_loop(&mut **body),
    &mut ProcCall(ref mut var, ref process, ref mut args) => visitor.visit_proc_call(var, process.clone(), args),
    &mut ExprStmt(ref mut expr) => visitor.visit_expr_stmt(expr),
    &mut QFUniverse(ref mut body) => visitor.visit_qf_universe(&mut **body),
    &mut Universe(ref mut queue, ref mut body) => visitor.visit_universe(queue, &mut **body),
    &mut Nothing => visitor.visit_nothing(),
  }
}

pub fn walk_stmts_mut<H, V: ?Sized>(visitor: &mut V, stmts: &mut Vec<Stmt>) where
  V: VisitorMut<H>
{
  for stmt in stmts {
    visitor.visit_stmt(stmt);
  }
}

pub fn walk_expr_mut<H, V: ?Sized>(visitor: &mut V, expr: &mut Expr) where
  V: VisitorMut<H>
{
  match &mut expr.node {
    &mut Number(value) => visitor.visit_number(value),
    &mut StringLiteral(ref value) => visitor.visit_string_lit(value.clone()),
    &mut NewInstance(ref mut new_instance) => visitor.visit_new_instance(new_instance.ty.clone(), &mut new_instance.args),
    &mut Call(ref mut call) => visitor.visit_method_call(call),
    // Bonsai expressions
    &mut Var(ref mut var) => visitor.visit_var(var),
    &mut Bottom => visitor.visit_bot(),
    &mut Top => visitor.visit_top(),
    // Trilean
    &mut Trilean(ref value) => visitor.visit_trilean(value.clone()),
    &mut Or(ref mut left, ref mut right) => visitor.visit_trilean_or(&mut **left, &mut **right),
    &mut And(ref mut left, ref mut right) => visitor.visit_trilean_and(&mut **left, &mut **right),
    &mut Not(ref mut expr) => visitor.visit_trilean_not(&mut **expr),
    &mut Entailment(ref mut rel) => visitor.visit_entailment(&mut **rel)
  }
}

pub fn walk_exprs_mut<H, V: ?Sized>(visitor: &mut V, exprs: &mut Vec<Expr>) where
  V: VisitorMut<H>
{
  for expr in exprs {
    visitor.visit_expr(expr);
  }
}

pub fn walk_method_call_mut<H, V: ?Sized>(visitor: &mut V, call: &mut MethodCall) where
  V: VisitorMut<H>
{
  if let Some(ref mut target) = call.target {
    visitor.visit_var(target);
  }
  walk_exprs_mut(visitor, &mut call.args)
}

pub fn walk_binding_mut<H, V: ?Sized>(visitor: &mut V, binding: &mut Binding) where
  V: VisitorMut<H>
{
  if let &mut Some(ref mut expr) = &mut binding.expr {
    visitor.visit_expr(expr)
  }
}

pub fn walk_proc_call_mut<H, V: ?Sized>(visitor: &mut V, var: &mut Option<Variable>, args: &mut Vec<Variable>) where
  V: VisitorMut<H>
{
  if let &mut Some(ref mut var) = var {
    visitor.visit_var(var)
  }
  for arg in args {
    visitor.visit_var(arg);
  }
}

pub fn walk_suspend_mut<H, V: ?Sized>(visitor: &mut V, suspend: &mut SuspendStmt) where
  V: VisitorMut<H>
{
  visitor.visit_expr(&mut suspend.condition);
  visitor.visit_stmt(&mut *suspend.body)
}
