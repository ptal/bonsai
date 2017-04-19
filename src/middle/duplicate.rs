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

/// Check duplicate names of local variables and modules attributes.

use context::*;
use std::collections::HashMap;

pub fn duplicate<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  let duplicate = Duplicate::new(context);
  duplicate.analyse()
}

struct Duplicate<'a> {
  context: Context<'a>,
  dup_local_vars: HashMap<String, Span>,
  dup_mod_attrs: HashMap<String, Span>,
  dup_proc: HashMap<String, Span>,
}

impl<'a> Duplicate<'a> {
  pub fn new(context: Context<'a>) -> Self {
    Duplicate {
      context: context,
      dup_local_vars: HashMap::new(),
      dup_mod_attrs: HashMap::new(),
      dup_proc: HashMap::new(),
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

  fn reset_dup_local_vars(&mut self) {
    self.dup_local_vars = self.dup_mod_attrs.clone();
  }

  fn reset_dup_proc(&mut self) {
    self.dup_proc.clear();
  }

  // Note: Due to the borrowing of session, we cannot take `dups` as mutable (both are owned by `self`).
  fn duplicate(dups: &HashMap<String, Span>, session: &Session,
    name: String, span: Span, code: &str, what: &str) -> bool
  {
    match dups.get(&name) {
      Some(prev_span) => {
        session.struct_span_err_with_code(span,
          &format!("duplicate {} definitions with name `{}`", what, name.clone()),
          code)
        .span_label(span, &"duplicate definition")
        .span_label(*prev_span, &format!("previous definition of `{}` here", name.clone()))
        .emit();
        true
      }
      _ => false
    }
  }

  fn duplicate_mod_attrs(&mut self, attrs: Vec<ModuleAttribute>) {
    self.dup_mod_attrs.clear();
    for attr in attrs {
      let binding = attr.binding.base();
      let name = binding.name.clone();
      let err = Self::duplicate(&self.dup_mod_attrs, self.session(),
        name.clone(), attr.span, "E0002", "spacetime attribute");
      if !err { self.dup_mod_attrs.insert(name, attr.span); }
    }
  }

  fn duplicate_local_vars(&mut self, let_stmt: &LetStmt) {
    let binding = let_stmt.binding.base();
    let name = binding.name.clone();
    let err = Self::duplicate(&self.dup_local_vars, self.session(),
      name.clone(), let_stmt.span, "E0003", "local variable");
    if !err { self.dup_local_vars.insert(name, let_stmt.span); }
  }

  fn duplicate_proc(&mut self, process: &Process) {
    let name = process.name.clone();
    let err = Self::duplicate(&self.dup_proc, self.session(),
      name.clone(), process.span, "E0004", "process");
    if !err { self.dup_proc.insert(name, process.span); }
  }
}

impl<'a> Visitor<JClass, ()> for Duplicate<'a> {
  unit_visitor_impl!(bcrate, JClass);
  unit_visitor_impl!(sequence);
  unit_visitor_impl!(parallel);
  unit_visitor_impl!(space);
  unit_visitor_impl!(tell);
  unit_visitor_impl!(pause);
  unit_visitor_impl!(pause_up);
  unit_visitor_impl!(stop);
  unit_visitor_impl!(exit);
  unit_visitor_impl!(proc_call);
  unit_visitor_impl!(fn_call);
  unit_visitor_impl!(module_call);
  unit_visitor_impl!(nothing);
  unit_visitor_impl!(binding_base);

  fn visit_module(&mut self, module: JModule) {
    self.reset_dup_proc();
    self.duplicate_mod_attrs(module.attributes);
    walk_processes(self, module.processes);
  }

  fn visit_process(&mut self, process: Process) {
    self.duplicate_proc(&process);
    self.reset_dup_local_vars();
    self.visit_stmt(process.body);
  }

  fn visit_let(&mut self, let_stmt: LetStmt) {
    // Due to the functionalization of module, some of the attributes are also local variables, but not all—such as references.
    if !let_stmt.is_mod_attr {
      self.duplicate_local_vars(&let_stmt);
    }
    self.visit_stmt(*(let_stmt.body));
  }
}

