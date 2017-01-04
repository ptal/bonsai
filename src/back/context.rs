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

/// The context is useful for mapping variable names to their types. This is mainly used when generating the closures because, at that point of the generation process, we do not have access to the type of the variables.

/// TODO: this context does not take into account the scoping rules of variables.

use jast::*;
use std::collections::HashMap;

pub struct Context {
  name_to_bindings: HashMap<String, LetBindingBase>
}

impl Context {
  pub fn new(module: JModule) -> Self {
    let mut context = Context {
      name_to_bindings: HashMap::new()
    };
    for channel_attr in module.channel_attrs() {
      context.insert_binding(channel_attr.base());
    }
    context.visit_program(module);
    context
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
    use ast::Stmt::*;
    match stmt {
      Let(decl) => {
        let base_binding = decl.binding.base().clone();
        self.insert_binding(base_binding);
        self.visit_stmt(*decl.body);
      }
      Seq(branches)
    | Par(branches)
    | Space(branches) => self.visit_stmts(branches),
      When(_, stmt)
    | Trap(_, stmt)
    | Loop(stmt) => self.visit_stmt(*stmt),
      _ => ()
    }
  }

  pub fn type_of_var(&self, var: &StreamVar) -> JType {
    self.name_to_bindings.get(&var.name())
      .expect(&format!("Undeclared variable `{}`.", var.name()))
      .ty.clone()
  }

  pub fn is_bonsai_var(&self, name: &String) -> bool {
    self.name_to_bindings.contains_key(name)
  }
}