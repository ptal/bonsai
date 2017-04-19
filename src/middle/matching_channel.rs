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

/// We create a module, ensure that an entry point exists (`execute` method) and move module attributes as `let` declarations wrapping the `execute` code.

use context::*;

pub fn matching_channel<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  let matching_channel = MatchingChannel::new(context);
  matching_channel.analyse()
}

struct MatchingChannel<'a> {
  context: Context<'a>,
  current_mod: usize
}

impl<'a> MatchingChannel<'a> {
  pub fn new(context: Context<'a>) -> Self {
    MatchingChannel {
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

  fn find_attr_by_name(&self, module: &JModule,
    name: String) -> Option<LetBinding>
  {
    module.attributes.iter()
      .map(|attr| attr.binding.clone())
      .find(|binding| binding.base().name == name)
  }

  fn cmp_binding(&self, attr_a: LetBinding, attr_b: LetBinding,
      mod_a_name: String, mod_b_name: String)
  {
    use ast::LetBinding::*;
    match (attr_a, attr_b) {
      (InStore(_), InStore(_)) => panic!(
        "Binding of the form `x = s <- e` cannot be channeled.\
        It occurs in module {} when instantiating module {}.",
        mod_a_name, mod_b_name),
      (Spacetime(b1), Spacetime(b2)) => self.cmp_spacetime_binding(b1, b2, mod_a_name, mod_b_name),
      (Module(b1), Module(b2)) => self.cmp_base_binding(b1.binding, b2.binding, mod_a_name, mod_b_name),
      (_,_) => panic!(
        "Incompatible matching with the channel variable.\
         It occurs in module {} when instantiating module {}.",
         mod_a_name, mod_b_name)
    }
  }

  fn cmp_spacetime_binding(&self, b1: LetBindingSpacetime, b2: LetBindingSpacetime,
    mod_a_name: String, mod_b_name: String)
  {
    let msg_err = |specifier| panic!(
      "{} specifier must match the one of the channel variable.\
       It occurs in module {} when instantiating module {}.",
       specifier, mod_a_name, mod_b_name);
    if b1.spacetime != b2.spacetime {
      msg_err("Spacetime");
    }
    if b1.is_transient != b2.is_transient {
      msg_err("Transient");
    }
    self.cmp_base_binding(b1.binding, b2.binding, mod_a_name.clone(), mod_b_name.clone());
  }

  fn cmp_base_binding(&self, b1: LetBindingBase, b2: LetBindingBase,
    mod_a_name: String, mod_b_name: String)
  {
    if b1.ty != b2.ty {
      panic!(
        "Type of variables must be the same during instantiation. \
        It occurs in module {} with type {} when instantiating module {} with type {}.",
         mod_a_name, b1.ty, mod_b_name, b2.ty);
    }
  }
}

impl<'a> Visitor<JClass, ()> for MatchingChannel<'a> {
  unit_visitor_impl!(module, JClass);
  unit_visitor_impl!(all_stmt);

  fn visit_crate(&mut self, bcrate: JCrate) {
    for (i, module) in bcrate.modules.into_iter().enumerate() {
      self.current_mod = i;
      self.visit_module(module);
    }
  }

  fn visit_binding_module(&mut self, mod_binding: LetBindingModule) {
    let mod_a = self.ast().modules[self.current_mod].clone();
    let mod_a_name = mod_a.file.mod_name();
    let mod_b_name = mod_binding.module_name();
    let mod_b = self.ast().find_mod_by_name(mod_b_name.clone());
    if mod_b.is_none() {
      let sp = mod_binding.binding.ty.span;
      self.session().struct_span_err_with_code(sp,
        &format!("Cannot find bonsai module `{}`.", mod_b_name.clone()),
        "E0001")
      .span_label(sp, &format!("not found in this scope"))
      .emit();
    }
    else {
      let mod_b = mod_b.unwrap();
      let channel_attrs = mod_b.channel_attrs();
      for attr_b in channel_attrs {
        let attr_a = self.find_attr_by_name(&mod_a, attr_b.base().name);
        let attr_a = attr_a.expect(&format!(
          "The module attribute {} could not be found in {} but is marked with `channel` in {}.",
          attr_b.base().name, mod_a_name.clone(), mod_b_name.clone()));
        self.cmp_binding(attr_a, attr_b, mod_a_name.clone(), mod_b_name.clone());
      }
    }
  }
}
