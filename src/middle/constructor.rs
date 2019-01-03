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

/// `Constructor` performs:
///  (a) Check that each module containing referenced fields (keyword `ref`) must have a single constructor initializing these variables.
///      It must simply appears in the constructor argument list with the same type and same name.
///      An assert checking that the argument and the field are the same is added in the generation stage.
///  (b) Register the module in the context with its associated constructor `ref` parameters list.

use context::*;
use session::*;

pub fn constructor(session: Session, context: Context) -> Env<Context> {
  let constructor = Constructor::new(session, context);
  constructor.analyse()
}

struct Constructor {
  session: Session,
  context: Context,
}

impl Constructor {
  pub fn new(session: Session, context: Context) -> Self {
    Constructor {
      session: session,
      context: context,
    }
  }

  fn session<'a>(&'a mut self) -> &'a mut Session {
    &mut self.session
  }

  fn analyse(mut self) -> Env<Context> {
    let bcrate_clone = self.context.clone_ast();
    self.visit_crate(bcrate_clone);
    if self.session.has_errors() {
      Env::fake(self.session, self.context)
    } else {
      Env::value(self.session, self.context)
    }
  }

  fn register_module(&mut self, module: &JModule) {
    self.context.alloc_module(module.mod_name());
    let mod_info = self.context.module_by_name_mut(module.mod_name());
    // Retrieve the position of each ref field in the constructor.
    let java_constructors = module.host.java_constructors.clone();
    if java_constructors.len() == 1 {
      mod_info.cons_len = java_constructors[0].parameters.len();
      mod_info.constructor = java_constructors[0]
        .parameters.iter()
        .enumerate()
        .filter_map(|(i, p)| module.ref_fields().into_iter().find(|f| f.binding.name == p.name).map(|f| (i, f)))
        .map(|(i, f)| (i, f.binding.uid))
        .collect();
    }
  }

  fn constructor(&mut self, module: &JModule) {
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

  fn match_constructor_ref(&mut self, ref_fields: Vec<ModuleField>, constructor: JConstructor) {
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

  /// Design rational: It simplifies the verification of module. Otherwise, we would need to look if the contructor calls other constructors (`this(...)` call inside a constructor) with the same `ref` parameters.
  fn err_multiple_constructor(&mut self, module: &JModule) {
    let constructors = module.host.java_constructors.clone();
    let help_msg = self.constructor_help_msg();
    self.session().struct_span_err_with_code(constructors[1].span,
      &format!("multiple constructors in the module `{}`.", module.host.class_name),
      "E0012")
    .span_label(constructors[0].span, &"previous constructor here")
    .help(&help_msg)
    .emit();
  }

  fn err_missing_constructor(&mut self, module: &JModule) {
    let help_msg = self.constructor_help_msg();
    self.session().struct_span_err_with_code(module.host.class_name.span,
      &format!("missing constructor in the module `{}`.", module.host.class_name),
      "E0013")
    .help(&help_msg)
    .emit();
  }

  fn err_missing_ref_param(&mut self, ref_field: &ModuleField, constructor: &JConstructor) {
    let help_msg = self.constructor_help_msg();
    self.session().struct_span_err_with_code(constructor.span,
      &format!("missing parameter in the constructor for initializing the field `{}`.", ref_field.binding.name),
      "E0014")
    .span_label(ref_field.binding.name.span, &"declared here")
    .help(&help_msg)
    .emit();
  }

  fn err_mismatch_ref_type(&mut self, ref_field: &ModuleField, param: &JParameter) {
    let binding = ref_field.binding.clone();
    let help_msg = self.constructor_help_msg();
    self.session().struct_span_err_with_code(param.span,
      &format!("Module field `{}` has type `{}` but was referenced with type `{}` in the constructor.",
        binding.name, binding.ty, param.ty),
      "E0015")
    .help(&help_msg)
    .emit();
  }

  fn constructor_help_msg(&self) -> String {
    format!("Module with `ref` fields must have a unique constructor.\n\
            This constructor must initialized the `ref` fields \
            and list them in the parameter list with the same names and types.")
  }
}

impl Visitor<JClass> for Constructor
{
  fn visit_module(&mut self, module: JModule) {
    self.register_module(&module);
    self.constructor(&module);
  }
}
