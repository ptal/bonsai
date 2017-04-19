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

pub use ast::*;
pub use session::*;
pub use visitor::*;
pub use partial::*;
use driver::config::Config;
use std::collections::HashMap;
use std::ops::Deref;

pub struct Context<'a> {
  pub session: &'a Session,
  pub ast: JCrate,
  // For each variable, compute the maximum number of `pre` that can possibly happen. This is useful to compute the size of the stream. For example: `pre pre x` gives `[x: 2]`.
  pub stream_bound: HashMap<String, usize>,
  pub name_to_bindings: HashMap<String, LetBindingBase>,
}

impl<'a> Context<'a> {
  pub fn new(session: &'a Session, ast: JCrate) -> Self {
    Context {
      session: session,
      ast: ast,
      stream_bound: HashMap::new(),
      name_to_bindings: HashMap::new()
    }
  }

  pub fn config(&self) -> &'a Config {
    self.session.config()
  }

  pub fn clone_ast(&self) -> JCrate {
    self.ast.clone()
  }

  pub fn init_module(&mut self, module: JModule) {
    self.name_to_bindings.clear();
    for channel_attr in module.channel_attrs() {
      self.insert_binding(channel_attr.base());
    }
    self.visit_program(module);
  }

  fn visit_program(&mut self, module: JModule) {
    for process in module.processes {
      self.visit_stmt(process.body);
    }
  }

  fn insert_binding(&mut self, binding: LetBindingBase) {
    self.name_to_bindings.insert(
      binding.name.clone(),
      binding);
  }

  fn visit_stmts(&mut self, stmts: Vec<Stmt>) {
    for stmt in stmts {
      self.visit_stmt(stmt);
    }
  }

  fn visit_stmt(&mut self, stmt: Stmt) {
    use ast::StmtKind::*;
    match stmt.node {
      Let(decl) => {
        let base_binding = decl.binding.base().clone();
        self.insert_binding(base_binding);
        self.visit_stmt(*decl.body);
      }
      Seq(branches)
    | Par(branches)
    | Space(branches) => self.visit_stmts(branches),
      When(_, stmt)
    | Suspend(_, stmt)
    | Trap(_, stmt)
    | Loop(stmt) => self.visit_stmt(*stmt),
      _ => ()
    }
  }

  pub fn binding_of(&self, name: &String) -> LetBindingBase {
    self.name_to_bindings.get(name)
    .expect(&format!("Undeclared variable `{}`.", name))
    .clone()
  }

  pub fn type_of_var(&self, var: &StreamVar) -> JType {
    self.binding_of(&var.name()).ty.clone()
  }

  pub fn is_bonsai_var(&self, name: &String) -> bool {
    self.name_to_bindings.contains_key(name)
  }

  pub fn stream_bound_of(&self, name: &String) -> usize {
    *self.stream_bound.get(name)
    .expect(&format!("Undeclared variable `{}`.", name))
  }
}

impl<'a> Deref for Context<'a> {
  type Target = Session;

  fn deref(&self) -> &Session {
    self.session
  }
}
