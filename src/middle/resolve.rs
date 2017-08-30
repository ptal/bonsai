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

/// Try to resolve path of variables such as `m.x.y`. It performs the following actions:
///
/// (1) Verify that the fields called on modules exist.
/// (2) Verify that processes called on modules exist.
/// (3) Compute the UID of path variables.

use context::*;
use session::*;

pub fn resolve(session: Session, context: Context) -> Env<Context> {
  let resolve = Resolve::new(session, context);
  resolve.analyse()
}

struct Resolve {
  session: Session,
  context: Context,
  current_mod: usize
}

impl Resolve {
  pub fn new(session: Session, context: Context) -> Self {
    Resolve {
      session: session,
      context: context,
      current_mod: 0
    }
  }

  fn session<'a>(&'a self) -> &'a Session {
    &self.session
  }

  fn analyse(mut self) -> Env<Context> {
    let mut bcrate_clone = self.context.clone_ast();
    self.visit_crate(&mut bcrate_clone);
    self.context.replace_ast(bcrate_clone);
    if self.session.has_errors() {
      Env::nothing(self.session)
    } else {
      Env::value(self.session, self.context)
    }
  }

  fn find_mod(&self, info: &VarInfo) -> JModule {
    self.context.ast
      .find_mod_by_name(&info.mod_name())
      .expect("[BUG] Every module variable is supposed to exist (analysis in `undeclared.rs`)")
  }

  fn resolve_path(&mut self, var: &mut Variable) {
    let mut i = 1;
    // At each iteration, we check if the field `var[i]` is a field of the module given by the type of `var[i-1]`.
    while i < var.len() {
      let info = self.context.var_by_uid(var.path.uids[i-1]);
      // If the current variable is not a bonsai module, we stop the resolution.
      // This marks the limit between Bonsai and its host language.
      if info.kind != Kind::Product {
        break;
      }
      let module = self.find_mod(&info);
      let field_name = var.path.fragments[i].clone();
      match module.find_field_by_name(&field_name) {
        Some(field) => {
          var.path.uids[i] = field.binding.uid;
        }
        None => {
          self.err_unknown_field(module, field_name);
          break;
        }
      }
      i += 1;
    }
  }

  fn resolve_process(&mut self, var: &mut Variable, process: Ident) {
    let target_uid = *var.path.uids.last().unwrap();
    if target_uid != 0 { // It is equals to 0 if `resolve_path` failed.
      let info = self.context.var_by_uid(target_uid);
      if info.kind != Kind::Product {
        self.err_foreign_process_call(&info, process.clone());
      }
      else {
        let module = self.find_mod(&info);
        if let None = module.find_process_by_name(&process) {
          self.err_unknown_process(module, process);
        }
      }
    }
  }

  fn err_unknown_field(&mut self, module: JModule, field: Ident) {
    self.err_unknown_item(module, field, "field", "E0008");
  }

  fn err_unknown_process(&mut self, module: JModule, process: Ident) {
    self.err_unknown_item(module, process, "process", "E0009");
  }

  fn err_unknown_item(&mut self, module: JModule, item: Ident,
    name_of_item: &str, code: &str)
  {
    self.session().struct_span_err_with_code(item.span,
      &format!("no {} `{}` in module `{}`.", name_of_item, item.clone(), module.mod_name()),
      code)
    .span_label(item.span, &format!("unknown {}", name_of_item))
    .emit();
  }

  fn err_foreign_process_call(&mut self, info: &VarInfo, process: Ident) {
    self.session().struct_span_err_with_code(process.span,
      &format!("forbidden call of the process `{}` on type `{}` because it is not a Bonsai module.",
        process.clone(), info.mod_name()),
      "E0010")
    .span_label(process.span, &format!("not a process"))
    .emit();
  }
}

impl VisitorMut<JClass> for Resolve
{
  fn visit_crate(&mut self, bcrate: &mut JCrate) {
    for (i, module) in bcrate.modules.iter_mut().enumerate() {
      self.current_mod = i;
      self.visit_module(module);
    }
  }

  fn visit_var(&mut self, var: &mut Variable) {
    self.resolve_path(var);
  }

  fn visit_proc_call(&mut self, var: &mut Option<Variable>, process: Ident) {
    if let &mut Some(ref mut var) = var {
      self.visit_var(var);
      self.resolve_process(var, process);
    }
  }
}

