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

use context::*;

pub fn matching_ref<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  let matching_ref = MatchingRef::new(context);
  matching_ref.analyse()
}

struct MatchingRef<'a> {
  context: Context<'a>,
  current_mod: usize
}

impl<'a> MatchingRef<'a> {
  pub fn new(context: Context<'a>) -> Self {
    MatchingRef {
      context: context,
      current_mod: 0
    }
  }

  fn session(&'a self) -> &'a Session {
    self.context.session
  }

  fn ast(&'a self) -> &'a JCrate {
    &self.context.ast
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

  fn find_field_by_name(&self, module: &JModule,
    name: String) -> Option<Binding>
  {
    module.fields.iter()
      .map(|field| field.binding.clone())
      .find(|binding| binding.name == name)
  }

  fn cmp_binding(&self, field_a: Binding, field_b: Binding,
      mod_a_name: String, mod_b_name: String)
  {
    let msg_err = |specifier| panic!(
      "{} specifier must match the one of the `ref` variable.\
       It occurs in module {} when instantiating module {}.",
       specifier, mod_a_name, mod_b_name);
    if field_a.is_transient() != field_b.is_transient() {
      msg_err("Transient");
    }
    if field_a.kind != field_b.kind {
      msg_err("Spacetime");
    }
    self.cmp_base_binding(field_a, field_b,
      mod_a_name.clone(), mod_b_name.clone());
  }

  fn cmp_base_binding(&self, field_a: Binding, field_b: Binding,
    mod_a_name: String, mod_b_name: String)
  {
    if field_a.ty != field_b.ty {
      panic!(
        "Type of variables must be the same during instantiation. \
        It occurs in module {} with type {} when instantiating module {} with type {}.",
         mod_a_name, field_a.ty, mod_b_name, field_b.ty);
    }
  }
}

impl<'a> Visitor<JClass> for MatchingRef<'a>
{
  fn visit_crate(&mut self, bcrate: JCrate) {
    for (i, module) in bcrate.modules.into_iter().enumerate() {
      self.current_mod = i;
      self.visit_module(module);
    }
  }

  fn visit_binding(&mut self, binding: Binding) {
    if binding.is_module() {
      let mod_a = self.ast().modules[self.current_mod].clone();
      let mod_a_name = mod_a.file.mod_name();
      let mod_b_name = binding.ty.name.clone();
      let mod_b = self.ast().find_mod_by_name(mod_b_name.clone());
      if mod_b.is_none() {
        let sp = binding.ty.span;
        self.session().struct_span_err_with_code(sp,
          &format!("Cannot find bonsai module `{}`.", mod_b_name.clone()),
          "E0001")
        .span_label(sp, &format!("not found in this scope"))
        .emit();
      }
      else {
        let mod_b = mod_b.unwrap();
        let ref_fields = mod_b.ref_fields();
        for field_b in ref_fields {
          let field_a = self.find_field_by_name(&mod_a, field_b.name.clone());
          let field_a = field_a.expect(&format!(
            "The module attribute {} could not be found in {} but is marked with `ref` in {}.",
            field_b.name.clone(), mod_a_name.clone(), mod_b_name.clone()));
          self.cmp_binding(field_a, field_b, mod_a_name.clone(), mod_b_name.clone());
        }
      }
    }
  }
}
