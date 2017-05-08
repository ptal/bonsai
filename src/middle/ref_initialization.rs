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

/// `RefInitialization` checks:
///  (a) Each module containing referenced fields (keyword `ref`) must has a constructor initializing this variable. It must simply appears in the constructor argument list with the same type and same name. An assert checking that the argument and the field are the same is added in the generation stage.
///  (b) Ref fields must not be initialized with right handed expression when declared.

use context::*;

pub fn ref_initialization<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  let ref_initialization = RefInitialization::new(context);
  ref_initialization.analyse()
}

struct RefInitialization<'a> {
  context: Context<'a>,
}

impl<'a> RefInitialization<'a> {
  pub fn new(context: Context<'a>) -> Self {
    RefInitialization {
      context: context,
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

  fn ref_fields(&self, module: &JModule) {
    for ref_field in module.ref_fields() {
      if ref_field.binding.expr.is_some() {
        self.err_initialized_ref_field(ref_field);
      }
    }
  }

  fn err_initialized_ref_field(&self, ref_field: ModuleField) {
    let binding = ref_field.binding;
    self.session().struct_span_err_with_code(ref_field.span,
      &format!("illegal initialization of `ref` field `{}`.", binding.name),
      "E0011")
    .span_label(ref_field.is_ref.unwrap(), &format!("illegal specifier"))
    .span_help(binding.expr.unwrap().span, &"Remove the initialization expression.")
    .emit();
  }
}

impl<'a> Visitor<JClass> for RefInitialization<'a>
{
  fn visit_module(&mut self, module: JModule) {
    self.ref_fields(&module);
  }
}
