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

pub fn resolve<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  let resolve = Resolve::new(context);
  resolve.analyse()
}

struct Resolve<'a> {
  context: Context<'a>,
  current_mod: usize
}

impl<'a> Resolve<'a> {
  pub fn new(context: Context<'a>) -> Self {
    Resolve {
      context: context,
      current_mod: 0
    }
  }

  fn session(&'a self) -> &'a Session {
    self.context.session
  }

  fn analyse(mut self) -> Partial<Context<'a>> {
    let mut bcrate_clone = self.context.clone_ast();
    self.visit_crate(&mut bcrate_clone);
    self.context.replace_ast(bcrate_clone);
    if self.session().has_errors() {
      Partial::Fake(self.context)
    } else {
      Partial::Value(self.context)
    }
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
      let module = self.context.ast
        .find_mod_by_name(&info.mod_name())
        .expect("[BUG] Every module variable is supposed to exist (analysis in `undeclared.rs`)");
      let field = var.path.fragments[i].clone();
      match module.find_field_by_name(&field) {
        Some(field) => {
          var.path.uids[i] = field.binding.uid;
        }
        None => {
          self.err_unknown_field(module, field);
          break;
        }
      }
      i += 1;
    }
  }

  fn err_unknown_field(&mut self, module: JModule, field: Ident) {
    self.session().struct_span_err_with_code(field.span,
      &format!("no field `{}` in module `{}`.", field.clone(), module.mod_name()),
      "E0008")
    .span_label(field.span, &format!("unknown field"))
    .emit();
  }
}

impl<'a> VisitorMut<JClass> for Resolve<'a>
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
}

