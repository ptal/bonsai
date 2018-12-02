// Copyright 2018 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::hash_set::HashSet;
use context::*;

pub fn free_variables(program: Stmt) -> HashSet<Variable> {
  let fv = FreeVariables::new();
  fv.collect(program)
}

struct FreeVariables {
  free_vars: HashSet<Variable>,
  in_scope_vars: HashSet<usize>
}

impl FreeVariables {
  pub fn new() -> Self {
    FreeVariables {
      free_vars: HashSet::new(),
      in_scope_vars: HashSet::new()
    }
  }

  fn collect(mut self, program: Stmt) -> HashSet<Variable> {
    self.visit_stmt(program);
    self.free_vars
  }

  fn enter_scope(&mut self, uid: usize) {
    self.in_scope_vars.insert(uid);
  }

  fn exit_scope(&mut self, uid: usize) {
    self.in_scope_vars.remove(&uid);
  }
}

impl Visitor<JClass> for FreeVariables
{
  fn visit_var(&mut self, var: Variable) {
    if self.in_scope_vars.contains(&var.first_uid()) {
      self.free_vars.insert(var);
    }
  }

  fn visit_let(&mut self, let_stmt: LetStmt) {
    let uid = let_stmt.binding.uid;
    self.enter_scope(uid);
    self.visit_binding(let_stmt.binding);
    self.visit_stmt(*(let_stmt.body));
    self.exit_scope(uid);
  }
}

