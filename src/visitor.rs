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

#![macro_use]

use ast::*;
use ast::Stmt::*;

pub trait Visitor<H, R>
{
  fn visit_crate(&mut self, bcrate: Crate<H>) -> R;

  fn visit_module(&mut self, module: Module<H>) -> R;

  fn visit_process(&mut self, process: Process) -> R {
    self.visit_stmt(process.body)
  }

  fn visit_stmt(&mut self, child: Stmt) -> R {
    walk_stmt(self, child)
  }

  fn visit_seq(&mut self, children: Vec<Stmt>) -> R;
  fn visit_par(&mut self, children: Vec<Stmt>) -> R;
  fn visit_space(&mut self, children: Vec<Stmt>) -> R;

  fn visit_let(&mut self, _binding: LetBinding, child: Stmt) -> R {
    self.visit_stmt(child)
  }

  fn visit_when(&mut self, _cond: EntailmentRel, child: Stmt) -> R {
    self.visit_stmt(child)
  }

  fn visit_tell(&mut self, _var: Var, _expr: Expr) -> R;
  fn visit_pause(&mut self) -> R;

  fn visit_trap(&mut self, _name: String, child: Stmt) -> R {
    self.visit_stmt(child)
  }

  fn visit_exit(&mut self, name: String) -> R;

  fn visit_loop(&mut self, child: Stmt) -> R {
    self.visit_stmt(child)
  }

  fn visit_proc_call(&mut self, name: String, args: Vec<Expr>) -> R;
  fn visit_fn_call(&mut self, expr: Expr) -> R;
  fn visit_nothing(&mut self) -> R;

  fn visit_binding_in_store(&mut self, binding: LetBindingBase, _store: String) -> R {
    self.visit_binding(binding)
  }

  fn visit_binding_spacetime(&mut self, binding: LetBindingBase, _spacetime: Spacetime) -> R {
    self.visit_binding(binding)
  }

  fn visit_binding_module(&mut self, binding: LetBindingBase) -> R {
    self.visit_binding(binding)
  }

  fn visit_binding(&mut self, binding: LetBindingBase) -> R;
}

pub fn walk_modules<H, R, V: ?Sized>(visitor: &mut V, modules: Vec<Module<H>>) -> Vec<R> where
  V: Visitor<H, R>
{
  modules.into_iter().map(|module| visitor.visit_module(module)).collect()
}

pub fn walk_processes<H, R, V: ?Sized>(visitor: &mut V, processes: Vec<Process>) -> Vec<R> where
  V: Visitor<H, R>
{
  processes.into_iter().map(|process| visitor.visit_process(process)).collect()
}

pub fn walk_stmt<H, R, V: ?Sized>(visitor: &mut V, stmt: Stmt) -> R where
  V: Visitor<H, R>
{
  match stmt {
    Seq(branches) => visitor.visit_seq(branches),
    Par(branches) => visitor.visit_par(branches),
    Space(branches) => visitor.visit_space(branches),
    Let(stmt) => visitor.visit_let(stmt.binding, *(stmt.body)),
    When(cond, body) => visitor.visit_when(cond, *body),
    Tell(var, expr) => visitor.visit_tell(var, expr),
    Pause => visitor.visit_pause(),
    Trap(name, body) => visitor.visit_trap(name, *body),
    Exit(name) => visitor.visit_exit(name),
    Loop(body) => visitor.visit_loop(*body),
    ProcCall(name, args) => visitor.visit_proc_call(name, args),
    FnCall(expr) => visitor.visit_fn_call(expr),
    Nothing => visitor.visit_nothing()
  }
}

pub fn walk_stmts<H, R, V: ?Sized>(visitor: &mut V, stmts: Vec<Stmt>) -> Vec<R> where
  V: Visitor<H, R>
{
  stmts.into_iter().map(|stmt| visitor.visit_stmt(stmt)).collect()
}

pub fn walk_binding<H, R, V: ?Sized>(visitor: &mut V, binding: LetBinding) -> R where
  V: Visitor<H, R>
{
  use ast::LetBinding::*;
  match binding {
    InStore(in_store) => visitor.visit_binding_in_store(in_store.binding, in_store.store),
    Spacetime(spacetime) => visitor.visit_binding_spacetime(spacetime.binding, spacetime.spacetime),
    Module(module) => visitor.visit_binding_module(module.binding)
  }
}

/// We need this macro for factorizing the code since we can not specialize a trait on specific type parameter (we would need to specialize on `()` here).
macro_rules! unit_visitor_impl {
  (module, H) => (
    fn visit_module(&mut self, module: Module<H>) {
      walk_processes(self, module.processes);
    }
  );
  (sequence) => (
    fn visit_seq(&mut self, children: Vec<Stmt>) {
      walk_stmts(self, children);
    }
  );
  (parallel) => (
    fn visit_par(&mut self, children: Vec<Stmt>) {
      walk_stmts(self, children);
    }
  );
  (space) => (
    fn visit_space(&mut self, children: Vec<Stmt>) {
      walk_stmts(self, children);
    }
  );
  (let_binding) => (
    fn visit_let(&mut self, binding: LetBinding, child: Stmt) {
      walk_binding(self, binding);
      self.visit_stmt(child);
    }
  );
  (binding_base) => (fn visit_binding(&mut self, _binding: LetBindingBase) {});
  (tell) => (fn visit_tell(&mut self, _var: Var, _expr: Expr) {});
  (pause) => (fn visit_pause(&mut self) {});
  (exit) => (fn visit_exit(&mut self, _name: String) {});
  (proc_call) => (fn visit_proc_call(&mut self, _name: String, _args: Vec<Expr>) {});
  (fn_call) => (fn visit_fn_call(&mut self, _expr: Expr) {});
  (nothing) => (fn visit_nothing(&mut self) {});
  (all, H) => (
    unit_visitor_impl!(module, H);
    unit_visitor_impl!(all_stmt);
  );
  (all_stmt) => (
    unit_visitor_impl!(sequence);
    unit_visitor_impl!(parallel);
    unit_visitor_impl!(space);
    unit_visitor_impl!(let_binding);
    unit_visitor_impl!(binding_base);
    unit_visitor_impl!(tell);
    unit_visitor_impl!(pause);
    unit_visitor_impl!(exit);
    unit_visitor_impl!(proc_call);
    unit_visitor_impl!(fn_call);
    unit_visitor_impl!(nothing);
  );
}
