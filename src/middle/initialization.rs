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

/// `Initialization` performs a static analysis to ensure the correct initialization of variables in the following contexts:
///  (a) Ref fields must not be initialized with right handed expression when declared. (E0011)
///  (b) Ref fields must be of the spacetime kind. (E0020)
///  (c) Local `module` variables:
///     (1) Module must always be initialized (E0019).
///     (2) If the module contains `ref` fields:
///       * It must be initialized with a `new` (E0018).
///       * `ref` arguments must be variables and not expressions (E0022).
///       * `ref` arguments must match the types and kinds of the corresponding module fields (E0023).
///
///  (d) `module` field variables: Module that contains refs variables cannot be initialized (E0021)
///  (e) Ref variables must not occurred when initializing field's RHS. (E0005)
///
///   Design rational:
///     (a) `ref` variables can only be retrieved from the environment, however it is not accessible when initializing the field.
///     (b) However `ref` variables can be retrieved in the constructor.
///   Future works:
///     * The well-formed initialization of (field) modules is left to the user. Indeed, we do not check what is the code in the Java constructor.

use context::*;
use session::*;

pub fn initialization(session: Session, context: Context) -> Env<Context> {
  let initialization = Initialization::new(session, context);
  initialization.analyse()
}

struct Initialization {
  session: Session,
  context: Context,
  current_mod: usize,
  visiting_fields: bool
}

impl Initialization {
  pub fn new(session: Session, context: Context) -> Self {
    Initialization {
      session: session,
      context: context,
      current_mod: 0,
      visiting_fields: false,
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

  fn ref_fields(&mut self, module: &JModule) {
    for ref_field in module.ref_fields() {
      if ref_field.binding.expr.is_some() {
        self.err_initialized_ref_field(&ref_field);
      }
      match ref_field.binding.kind {
        Kind::Product
      | Kind::Host => self.err_kind_ref_field(&ref_field),
        _ => ()
      }
    }
  }

  fn is_module_ref(&self, binding: &Binding) -> bool {
    self.context.module_by_name(binding.ty.name.clone()).has_refs()
  }

  fn module_field(&mut self, binding: &Binding) {
    if binding.expr.is_some() {
      if self.is_module_ref(binding) {
        self.err_module_ref_in_field(binding)
      }
    }
  }

  fn module_local_var(&mut self, binding: &Binding) {
    match binding.expr.clone() {
      Some(expr) => self.module_refs_initializer(binding, expr),
      None => self.err_module_missing_initializer(binding)
    }
  }

  fn module_refs_initializer(&mut self, binding: &Binding, expr: Expr) {
    if self.is_module_ref(binding) {
      match expr.node {
        ExprKind::NewInstance(new_instance) => {
          self.module_initialization_list(binding, new_instance.ty, new_instance.args);
        }
        _ => self.err_module_illegal_initializer(binding)
      }
    }
  }

  fn module_initialization_list(&mut self, binding: &Binding, ty: JType, args: Vec<Expr>) {
    let mod_info = self.context.module_by_name(ty.name);
    if mod_info.cons_len != args.len() {
      self.err_param_list_differ(binding, &mod_info, &args)
    }
    else {
      for (pos, uid) in mod_info.constructor {
        self.ref_instantiation(&args[pos], uid);
      }
    }
  }

  fn ref_instantiation(&mut self, expr: &Expr, uid: usize) {
    let ref_info = self.context.var_by_uid(uid);
    match expr.node.clone() {
      ExprKind::Var(var) => {
        let var_info = self.context.var_by_uid(var.last_uid());
        if var_info.kind != ref_info.kind || var_info.ty != ref_info.ty {
          self.err_mismatch_kind_type(expr.span, &var_info, &ref_info)
        }
      }
      _ => self.err_instantiate_ref_with_expr(&ref_info, expr)
    }
  }

  fn contains_ref(&self, var: &Variable) -> Option<Ident> {
    for i in 0..var.path.len() {
      if self.context.var_by_uid(var.path.uids[i]).is_ref() {
        return Some(var.path.fragments[i].clone())
      }
    }
    None
  }

  fn msg_expected_var(&self, var_info: &VarInfo) -> String {
    format!("expected variable of type `{}` and kind `{}`.", var_info.ty, var_info.kind)
  }

  fn err_param_list_differ(&mut self, binding: &Binding, mod_info: &ModuleInfo, args: &Vec<Expr>) {
    self.session().struct_span_err_with_code(binding.expr.as_ref().unwrap().span,
      &format!("constructor has size {} but was called with {} arguments.", mod_info.cons_len, args.len()),
      "E0024")
    .emit();
  }

  fn err_mismatch_kind_type(&mut self, sp: Span, var_info: &VarInfo, ref_info: &VarInfo) {
    let msg = self.msg_expected_var(ref_info);
    self.session().struct_span_err_with_code(sp,
      &msg,
      "E0023")
    .span_label(sp, &format!("has type `{}` and kind `{}`.", var_info.ty, var_info.kind))
    .emit();
  }

  fn err_instantiate_ref_with_expr(&mut self, var_info: &VarInfo, expr: &Expr) {
    let msg = self.msg_expected_var(var_info);
    self.session().struct_span_err_with_code(expr.span,
      &msg,
      "E0022")
    .span_help(expr.span,
      &format!("Illegal instantiation of a `ref` variable with an expression.\n\
               Create a local variable initialized to this expression."))
    .emit();
  }

  fn err_ref_var_in_field(&mut self, var: &Variable, first_ref: Ident) {
    let msg = self.msg_ref_field();
    self.session().struct_span_err_with_code(var.span,
      &format!("forbidden occurrence of `ref` variable `{}` when initializing a field.", first_ref),
      "E0005")
    .span_label(first_ref.span, &format!("illegal occurrence"))
    .help(&msg)
    .emit();
  }

  fn err_module_ref_in_field(&mut self, binding: &Binding) {
    let msg = self.msg_ref_field();
    self.session().struct_span_err_with_code(binding.span,
      &format!("forbidden initialization of the module `{}` in a field.", binding.ty.name),
      "E0021")
    .help(&msg)
    .emit();
  }

  fn msg_ref_field(&self) -> String {
    format!("At this stage, the `ref` variables are not yet initialized and are equal to `null`.\n\
             You can initialize this field in the constructor.")
  }

  fn err_initialized_ref_field(&mut self, ref_field: &ModuleField) {
    let binding = ref_field.binding.clone();
    self.session().struct_span_err_with_code(ref_field.span,
      &format!("illegal initialization of `ref` field `{}`.", binding.name),
      "E0011")
    .span_label(ref_field.is_ref.unwrap(), &format!("illegal specifier"))
    .span_help(binding.expr.unwrap().span, &"Remove the initialization expression.")
    .emit();
  }

  fn err_kind_ref_field(&mut self, ref_field: &ModuleField) {
    let binding = ref_field.binding.clone();
    self.session().struct_span_err_with_code(ref_field.span,
      &format!("illegal kind for the `ref` field `{}`.", binding.name),
      "E0020")
    .help(&"`ref` field must have the spacetime kind (`single_time`, `single_space` or `world_line`).")
    .emit();
  }

  fn err_module_illegal_initializer(&mut self, binding: &Binding) {
    let msg = self.msg_module_initializer();
    self.session().struct_span_err_with_code(binding.span,
      &format!("illegal initialization of the module variable `{}`.", binding.name),
      "E0018")
    .span_label(binding.expr.as_ref().unwrap().span, &"illegal initializer")
    .help(&msg)
    .emit();
  }

  fn err_module_missing_initializer(&mut self, binding: &Binding) {
    let msg = self.msg_module_initializer();
    self.session().struct_span_err_with_code(binding.span,
      &format!("missing initialization of the module variable `{}`.", binding.name),
      "E0019")
    .help(&msg)
    .emit();
  }

  fn msg_module_initializer(&self) -> String {
    format!("Module variables must be initialized with the `new` operator \
             (module field can be left uninitialized).")
  }

  fn err_host_local_var(&mut self, binding: &Binding) {
    self.session().struct_span_err_with_code(binding.span,
      &format!("missing spacetime specifier for the variable `{}`.", binding.name),
      "E0025")
    .help(&"Local variable must be either module or spacetime variable. \
            \"Pure\" Java variables can only occurs as fields of the module.\
            Adding a `single_space` specifier should give you the expected behavior.")
    .emit();
  }
}

impl Visitor<JClass> for Initialization
{
  fn visit_crate(&mut self, bcrate: JCrate) {
    for (i, module) in bcrate.modules.into_iter().enumerate() {
      self.current_mod = i;
      self.visit_module(module);
    }
  }

  fn visit_module(&mut self, module: JModule) {
    self.ref_fields(&module);
    self.visiting_fields = true;
    walk_fields(self, module.fields);
    self.visiting_fields = false;
    walk_processes(self, module.processes);
  }

  fn visit_binding(&mut self, binding: Binding) {
    if binding.is_module() {
      if self.visiting_fields {
        self.module_field(&binding);
      }
      else {
        self.module_local_var(&binding);
      }
    }
    else if binding.is_host() && !self.visiting_fields {
      self.err_host_local_var(&binding);
    }
    walk_binding(self, binding);
  }

  fn visit_var(&mut self, var: Variable) {
    if self.visiting_fields {
      if let Some(first_ref) = self.contains_ref(&var) {
        self.err_ref_var_in_field(&var, first_ref);
      }
    }
  }
}
