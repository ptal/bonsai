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

use jast::*;
use std::collections::HashMap;

struct SpacetimeVar {
  pub name: String,
  pub ty: JType
}

impl SpacetimeVar {
  pub fn new(name: String, ty: JType) -> Self {
    SpacetimeVar {
      name: name,
      ty: ty
    }
  }
}

pub struct Context {
  spacetime_vars: HashMap<String, SpacetimeVar>
}

impl Context {
  pub fn new(module: JModule) -> Self {
    let mut context = Context {
      spacetime_vars: HashMap::new()
    };
    context.initialize_program(module);
    context
  }

  fn initialize_program(&mut self, module: JModule) {
    for process in module.processes {
      self.initialize_stmt(process.body);
    }
  }

  fn insert_var(&mut self, var: String, ty: JType) {
    let spacetime_var = SpacetimeVar::new(var.clone(), ty);
    self.spacetime_vars.insert(
      var,
      spacetime_var);
  }

  fn initialize_stmts(&mut self, stmts: Vec<Stmt>) {
    for stmt in stmts {
      self.initialize_stmt(stmt);
    }
  }

  fn initialize_stmt(&mut self, stmt: Stmt) {
    use ast::Stmt::*;
    match stmt {
      Let(decl) => {
        self.insert_var(decl.var.name, decl.var.ty);
        self.initialize_stmt(*decl.body);
      }
      LetInStore(decl) => {
        self.insert_var(decl.var.name, decl.var.ty);
        self.initialize_stmt(*decl.body);
      }
      Seq(branches)
    | Par(branches)
    | Space(branches) => self.initialize_stmts(branches),
      When(_, stmt)
    | Trap(_, stmt)
    | Loop(stmt) => self.initialize_stmt(*stmt),
      _ => ()
    }
  }

  pub fn type_of_var(&self, var: &StreamVar) -> JType {
    self.spacetime_vars.get(&var.name)
      .expect(&format!("Undeclared variable `{}`.", var.name))
      .ty.clone()
  }

  pub fn is_spacetime_var(&self, name: &String) -> bool {
    self.spacetime_vars.contains_key(name)
  }
}