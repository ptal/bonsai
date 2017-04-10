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

use ast::*;
use visitor::*;
use partial::*;
use session::*;
use std::collections::HashMap;

pub fn duplicate<H: Clone>(session: &Session, bcrate: Crate<H>) -> Partial<Crate<H>> {
  let duplicate = Duplicate::new(session, bcrate);
  duplicate.analyse()
}

struct Duplicate<'a, H> {
  bcrate: Crate<H>,
  session: &'a Session,
  dup_local_vars: HashMap<String, Span>,
  dup_mod_attrs: HashMap<String, Span>
}

impl<'a, H: Clone> Duplicate<'a, H> {
  pub fn new(session: &'a Session, bcrate: Crate<H>) -> Self {
    Duplicate {
      bcrate: bcrate,
      session: session,
      dup_local_vars: HashMap::new(),
      dup_mod_attrs: HashMap::new(),
    }
  }

  fn analyse(mut self) -> Partial<Crate<H>> {
    let bcrate_clone = self.bcrate.clone();
    self.visit_crate(bcrate_clone);
    if self.session.has_errors() {
      Partial::Fake(self.bcrate)
    } else {
      Partial::Value(self.bcrate)
    }
  }

  fn duplicate_mod_attrs(&mut self, attrs: Vec<ModuleAttribute>) {
    self.dup_mod_attrs.clear();
    for attr in attrs {
      let binding = attr.binding.base();
      let err =
        match self.dup_mod_attrs.get(&binding.name) {
          Some(prev_span) => {
            self.session.struct_span_err_with_code(attr.span,
              &format!("Module attribute `{}` is already declared.", binding.name.clone()),
              "E0002")
            .span_label(attr.span, &"attribute already declared")
            .span_label(*prev_span, &format!("`{}` first declared here", binding.name.clone()))
            .emit();
            true
          }
          None => false
        };
      if !err { self.dup_mod_attrs.insert(binding.name, attr.span); }
    }
  }

  fn reset_dup_local_vars(&mut self) {
    self.dup_local_vars = self.dup_mod_attrs.clone();
  }

  fn duplicate_local_vars(&mut self, let_stmt: &LetStmt) {
    let binding = let_stmt.binding.base();
    let err =
      match self.dup_local_vars.get(&binding.name) {
        Some(prev_span) => {
          self.session.struct_span_err_with_code(let_stmt.span,
            &format!("Local variable `{}` is already declared.", binding.name.clone()),
            "E0003")
          .span_label(let_stmt.span, &"local variable already declared")
          .span_label(*prev_span, &format!("`{}` first declared here", binding.name.clone()))
          .emit();
          true
        }
        _ => false
      };
    if !err { self.dup_local_vars.insert(binding.name, let_stmt.span); }
  }
}

impl<'a, H: Clone> Visitor<H, ()> for Duplicate<'a, H> {
  unit_visitor_impl!(bcrate, H);
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

  fn visit_module(&mut self, module: Module<H>) {
    self.duplicate_mod_attrs(module.attributes);
    walk_processes(self, module.processes);
  }

  fn visit_process(&mut self, process: Process) {
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

