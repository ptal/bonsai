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

/// Check duplicate names of:
///  (1) Processes.
///  (2) Local variables per process.
///  (3) Spacetime fields in modules.

use context::*;
use std::collections::HashMap;

pub fn duplicate<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  let duplicate = Duplicate::new(context);
  duplicate.analyse()
}

struct Duplicate<'a> {
  context: Context<'a>,
  dup_local_vars: HashMap<String, Span>,
  dup_mod_fields: HashMap<String, Span>,
  dup_procs: HashMap<String, Span>,
}

impl<'a> Duplicate<'a> {
  pub fn new(context: Context<'a>) -> Self {
    Duplicate {
      context: context,
      dup_local_vars: HashMap::new(),
      dup_mod_fields: HashMap::new(),
      dup_procs: HashMap::new(),
    }
  }

  fn session(&'a self) -> &'a Session {
    self.context.session
  }

  fn analyse(mut self) -> Partial<Context<'a>> {
    let bcrate_clone = self.context.clone_ast();
    self.visit_crate(bcrate_clone);
    if self.session().has_errors() {
      Partial::Fake(self.context)
    } else {
      Partial::Value(self.context)
    }
  }

  fn reset_dup_mod_fields(&mut self) {
    self.dup_mod_fields.clear();
  }

  fn reset_dup_local_vars(&mut self) {
    self.dup_local_vars = self.dup_mod_fields.clone();
  }

  fn reset_dup_procs(&mut self) {
    self.dup_procs.clear();
  }

  // Note: Due to the borrowing of session, we cannot take `dups` as mutable (both are owned by `self`).
  fn duplicate(dups: &HashMap<String, Span>, session: &Session,
    name: Ident, code: &str, what: &str) -> bool
  {
    match dups.get(&*name) {
      Some(prev_span) => {
        session.struct_span_err_with_code(name.span,
          &format!("duplicate {} definitions with name `{}`", what, name.clone()),
          code)
        .span_label(name.span, &"duplicate definition")
        .span_label(*prev_span, &format!("previous definition of `{}` here", name.clone()))
        .emit();
        true
      }
      _ => false
    }
  }

  fn duplicate_field(&mut self, field: ModuleField) {
    let binding = field.binding.clone();
    let name = binding.name.clone();
    let err = Self::duplicate(&self.dup_mod_fields, self.session(),
      name.clone(), "E0002", "spacetime field");
    if !err { self.dup_mod_fields.insert(name.unwrap(), field.span); }
  }

  fn duplicate_local_var(&mut self, let_stmt: &LetStmt) {
    let binding = let_stmt.binding.clone();
    let name = binding.name.clone();
    let err = Self::duplicate(&self.dup_local_vars, self.session(),
      name.clone(), "E0003", "local variable");
    if !err { self.dup_local_vars.insert(name.unwrap(), let_stmt.span); }
  }

  fn duplicate_proc(&mut self, process: &Process) {
    let name = process.name.clone();
    let err = Self::duplicate(&self.dup_procs, self.session(),
      name.clone(), "E0004", "process");
    if !err { self.dup_procs.insert(name.unwrap(), process.span); }
  }
}

impl<'a> Visitor<JClass> for Duplicate<'a>
{
  fn visit_module(&mut self, module: JModule) {
    self.reset_dup_procs();
    self.reset_dup_mod_fields();
    walk_fields(self, module.fields);
    walk_processes(self, module.processes);
  }

  fn visit_field(&mut self, field: ModuleField) {
    self.duplicate_field(field);
  }

  fn visit_process(&mut self, process: Process) {
    self.reset_dup_local_vars();
    self.duplicate_proc(&process);
    self.visit_stmt(process.body);
  }

  fn visit_let(&mut self, let_stmt: LetStmt) {
    self.duplicate_local_var(&let_stmt);
    self.visit_stmt(*(let_stmt.body));
  }
}

