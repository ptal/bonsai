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

  fn visit_space(&mut self, children: Vec<Stmt>) {
    walk_stmts(self, children);
  }

  fn visit_let(&mut self, let_stmt: LetStmt) {
    self.visit_binding(let_stmt.binding);
    self.visit_stmt(*(let_stmt.body))
  }

  fn visit_when(&mut self, cond: Condition, child: Stmt) {
    self.visit_entailment(cond.unwrap());
    self.visit_stmt(child)
  }

  fn visit_suspend(&mut self, cond: Condition, child: Stmt) {
    self.visit_entailment(cond.unwrap());
    self.visit_stmt(child)
  }

  fn visit_tell(&mut self, var: StreamVar, expr: Expr) {
    self.visit_var(var);
    self.visit_expr(expr);
  }

  fn visit_pause(&mut self) {}
  fn visit_pause_up(&mut self) {}
  fn visit_stop(&mut self) {}

  fn visit_trap(&mut self, _name: String, child: Stmt) {
    self.visit_stmt(child)
  }

  fn visit_exit(&mut self, _name: String) {}

  fn visit_loop(&mut self, child: Stmt) {
    self.visit_stmt(child)
  }

  fn visit_proc_call(&mut self, _name: String, args: Vec<Expr>) {
    walk_exprs(self, args)
  }

  fn visit_fn_call(&mut self, expr: Expr) {
    self.visit_expr(expr)
  }

  fn visit_module_call(&mut self, _expr: RunExpr) {}
  fn visit_nothing(&mut self) {}

  fn visit_universe(&mut self, child: Stmt) {
    self.visit_stmt(child)
  }

  fn visit_binding(&mut self, binding: Binding) {
    self.visit_expr(binding.expr)
  }

  fn visit_entailment(&mut self, rel: EntailmentRel) {
    self.visit_var(rel.left);
    self.visit_expr(rel.right);
  }

  fn visit_expr(&mut self, expr: Expr) {
    walk_expr(self, expr)
  }

  fn visit_jnew(&mut self, _ty: JType, args: Vec<Expr>) {
    walk_exprs(self, args)
  }

  fn visit_object_call(&mut self, _object: String, calls: Vec<JavaCall>) {
    walk_jcalls(self, calls)
  }

  fn visit_this_call(&mut self, call: JavaCall) {
    self.visit_jcall(call);
  }

  fn visit_jcall(&mut self, call: JavaCall) {
    walk_exprs(self, call.args);
  }

  fn visit_boolean(&mut self, _value: bool) {}
  fn visit_number(&mut self, _value: u64) {}
  fn visit_string_lit(&mut self, _value: String) {}
  fn visit_var(&mut self, _var: StreamVar) {}
  fn visit_bot(&mut self, _ty: JType) {}
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
    Par(branches) => visitor.visit_par(branches),
    Space(branches) => visitor.visit_space(branches),
    Let(stmt) => visitor.visit_let(stmt),
    When(cond, body) => visitor.visit_when(cond, *body),
    Suspend(cond, body) => visitor.visit_suspend(cond, *body),
    Tell(var, expr) => visitor.visit_tell(var, expr),
    Pause => visitor.visit_pause(),
    PauseUp => visitor.visit_pause_up(),
    Stop => visitor.visit_stop(),
    Trap(name, body) => visitor.visit_trap(name, *body),
    Exit(name) => visitor.visit_exit(name),
    Loop(body) => visitor.visit_loop(*body),
    ProcCall(name, args) => visitor.visit_proc_call(name, args),
    FnCall(expr) => visitor.visit_fn_call(expr),
    ModuleCall(expr) => visitor.visit_module_call(expr),
    Universe(body) => visitor.visit_universe(*body),
    Nothing => visitor.visit_nothing()
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
    JavaNew(ty, args) => visitor.visit_jnew(ty, args),
    JavaObjectCall(object, calls) => visitor.visit_object_call(object, calls),
    JavaThisCall(call) => visitor.visit_this_call(call),
    Boolean(value) => visitor.visit_boolean(value),
    Number(value) => visitor.visit_number(value),
    StringLiteral(value) => visitor.visit_string_lit(value),
    Variable(var) => visitor.visit_var(var),
    Bottom(ty) => visitor.visit_bot(ty)
  }
}

pub fn walk_exprs<H, V: ?Sized>(visitor: &mut V, exprs: Vec<Expr>) where
  V: Visitor<H>
{
  for expr in exprs {
    visitor.visit_expr(expr);
  }
}

pub fn walk_jcalls<H, V: ?Sized>(visitor: &mut V, jcalls: Vec<JavaCall>) where
  V: Visitor<H>
{
  for jcall in jcalls {
    visitor.visit_jcall(jcall);
  }
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

  fn visit_space(&mut self, children: &mut Vec<Stmt>) {
    walk_stmts_mut(self, children);
  }

  fn visit_let(&mut self, let_stmt: &mut LetStmt) {
    self.visit_binding(&mut let_stmt.binding);
    self.visit_stmt(&mut *(let_stmt.body))
  }

  fn visit_when(&mut self, cond: &mut Condition, child: &mut Stmt) {
    self.visit_entailment(cond.unwrap_mut());
    self.visit_stmt(child)
  }

  fn visit_suspend(&mut self, cond: &mut Condition, child: &mut Stmt) {
    self.visit_entailment(cond.unwrap_mut());
    self.visit_stmt(child)
  }

  fn visit_tell(&mut self, var: &mut StreamVar, expr: &mut Expr) {
    self.visit_var(var);
    self.visit_expr(expr);
  }

  fn visit_pause(&mut self) {}
  fn visit_pause_up(&mut self) {}
  fn visit_stop(&mut self) {}

  fn visit_trap(&mut self, _name: String, child: &mut Stmt) {
    self.visit_stmt(child)
  }

  fn visit_exit(&mut self, _name: String) {}

  fn visit_loop(&mut self, child: &mut Stmt) {
    self.visit_stmt(child)
  }

  fn visit_proc_call(&mut self, _name: String, args: &mut Vec<Expr>) {
    walk_exprs_mut(self, args)
  }

  fn visit_fn_call(&mut self, expr: &mut Expr) {
    self.visit_expr(expr)
  }

  fn visit_module_call(&mut self, _expr: &mut RunExpr) {}
  fn visit_nothing(&mut self) {}

  fn visit_universe(&mut self, child: &mut Stmt) {
    self.visit_stmt(child)
  }

  fn visit_binding(&mut self, binding: &mut Binding) {
    self.visit_expr(&mut binding.expr)
  }

  fn visit_expr(&mut self, expr: &mut Expr) {
    walk_expr_mut(self, expr)
  }

  fn visit_jnew(&mut self, _ty: JType, args: &mut Vec<Expr>) {
    walk_exprs_mut(self, args)
  }

  fn visit_object_call(&mut self, _object: String, calls: &mut Vec<JavaCall>) {
    walk_jcalls_mut(self, calls)
  }

  fn visit_this_call(&mut self, call: &mut JavaCall) {
    self.visit_jcall(call);
  }

  fn visit_jcall(&mut self, call: &mut JavaCall) {
    walk_exprs_mut(self, &mut call.args);
  }

  fn visit_entailment(&mut self, rel: &mut EntailmentRel) {
    self.visit_var(&mut rel.left);
    self.visit_expr(&mut rel.right);
  }

  fn visit_var(&mut self, _var: &mut StreamVar) {}

  fn visit_boolean(&mut self, _value: bool) {}
  fn visit_number(&mut self, _value: u64) {}
  fn visit_string_lit(&mut self, _value: String) {}
  fn visit_bot(&mut self, _ty: JType) {}
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
    &mut Par(ref mut branches) => visitor.visit_par(branches),
    &mut Space(ref mut branches) => visitor.visit_space(branches),
    &mut Let(ref mut stmt) => visitor.visit_let(stmt),
    &mut When(ref mut cond, ref mut body) => visitor.visit_when(cond, &mut **body),
    &mut Suspend(ref mut cond, ref mut body) => visitor.visit_suspend(cond, &mut **body),
    &mut Tell(ref mut var, ref mut expr) => visitor.visit_tell(var, expr),
    &mut Pause => visitor.visit_pause(),
    &mut PauseUp => visitor.visit_pause_up(),
    &mut Stop => visitor.visit_stop(),
    &mut Trap(ref mut name, ref mut body) => visitor.visit_trap(name.clone(), &mut **body),
    &mut Exit(ref mut name) => visitor.visit_exit(name.clone()),
    &mut Loop(ref mut body) => visitor.visit_loop(&mut **body),
    &mut ProcCall(ref mut name, ref mut args) => visitor.visit_proc_call(name.clone(), args),
    &mut FnCall(ref mut expr) => visitor.visit_fn_call(expr),
    &mut ModuleCall(ref mut expr) => visitor.visit_module_call(expr),
    &mut Universe(ref mut body) => visitor.visit_universe(&mut **body),
    &mut Nothing => visitor.visit_nothing()
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
    &mut JavaNew(ref ty, ref mut args) => visitor.visit_jnew(ty.clone(), args),
    &mut JavaObjectCall(ref object, ref mut calls) => visitor.visit_object_call(object.clone(), calls),
    &mut JavaThisCall(ref mut call) => visitor.visit_this_call(call),
    &mut Boolean(value) => visitor.visit_boolean(value),
    &mut Number(value) => visitor.visit_number(value),
    &mut StringLiteral(ref value) => visitor.visit_string_lit(value.clone()),
    &mut Variable(ref mut var) => visitor.visit_var(var),
    &mut Bottom(ref ty) => visitor.visit_bot(ty.clone())
  }
}

pub fn walk_exprs_mut<H, V: ?Sized>(visitor: &mut V, exprs: &mut Vec<Expr>) where
  V: VisitorMut<H>
{
  for expr in exprs {
    visitor.visit_expr(expr);
  }
}

pub fn walk_jcalls_mut<H, V: ?Sized>(visitor: &mut V, jcalls: &mut Vec<JavaCall>) where
  V: VisitorMut<H>
{
  for jcall in jcalls {
    visitor.visit_jcall(jcall);
  }
}

