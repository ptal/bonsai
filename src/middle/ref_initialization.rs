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
///  (a) Each module containing referenced fields (keyword `ref`) must has a single constructor initializing these variables. It must simply appears in the constructor argument list with the same type and same name. An assert checking that the argument and the field are the same is added in the generation stage (TODO).
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

  fn constructor(&self, module: &JModule) {
    let ref_fields = module.ref_fields();
    // There are requirements on the constructors only if the module has `ref` fields.
    if ref_fields.len() > 0 {
      if module.host.java_constructors.len() > 1 {
        self.err_multiple_constructor(module);
      }
      else if module.host.java_constructors.len() == 0 {
        self.err_missing_constructor(module);
      }
      else {
        let constructor = module.host.java_constructors[0].clone();
        self.match_constructor_ref(ref_fields, constructor);
      }
    }
  }

  fn match_constructor_ref(&self, ref_fields: Vec<ModuleField>, constructor: JConstructor) {
    for ref_field in ref_fields {
      let param = constructor.parameters.iter().find(|p| p.name == ref_field.binding.name);
      match param {
        None => self.err_missing_ref_param(&ref_field, &constructor),
        Some(param) => {
          if param.ty != ref_field.binding.ty {
            self.err_mismatch_ref_type(&ref_field, &param);
          }
        }
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

  /// Design rational: It simplifies the verification of module. Otherwise, we would need to look if the contructor calls other constructors (`this(...)` call inside a constructor) with the same `ref` parameters.
  fn err_multiple_constructor(&self, module: &JModule) {
    let constructors = module.host.java_constructors.clone();
    self.session().struct_span_err_with_code(constructors[1].span,
      &format!("multiple constructors in the module `{}`.", module.host.class_name),
      "E0012")
    .span_label(constructors[0].span, &"previous constructor here")
    .help(&self.constructor_help_msg())
    .emit();
  }

  fn err_missing_constructor(&self, module: &JModule) {
    self.session().struct_span_err_with_code(module.host.class_name.span,
      &format!("missing constructor in the module `{}`.", module.host.class_name),
      "E0013")
    .help(&self.constructor_help_msg())
    .emit();
  }

  fn err_missing_ref_param(&self, ref_field: &ModuleField, constructor: &JConstructor) {
    self.session().struct_span_err_with_code(constructor.span,
      &format!("missing `ref` parameter `{}` in the constructor.", ref_field.binding.name),
      "E0014")
    .span_label(ref_field.binding.name.span, &"declared here")
    .help(&self.constructor_help_msg())
    .emit();
  }

  fn err_mismatch_ref_type(&self, ref_field: &ModuleField, param: &JParameter) {
    let binding = ref_field.binding.clone();
    self.session().struct_span_err_with_code(param.span,
      &format!("Module field `{}` has type `{}` but was referenced with type `{}` in the constructor.",
        binding.name, binding.ty, param.ty),
      "E0015")
    .help(&self.constructor_help_msg())
    .emit();
  }

  fn constructor_help_msg(&self) -> String {
    format!("Module with `ref` fields must have a unique constructor.\n\
            This constructor must initialized the `ref` fields \
            and list them in the parameter list with the same names and types.")
  }
}

impl<'a> Visitor<JClass> for RefInitialization<'a>
{
  fn visit_module(&mut self, module: JModule) {
    self.ref_fields(&module);
    self.constructor(&module);
  }
}
