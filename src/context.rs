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

pub use ast::*;
pub use visitor::*;
pub use partial::*;

pub struct Context {
  pub ast: JCrate,
  pub vars: Vec<VarInfo>,
  pub modules: Vec<ModuleInfo>
}

#[derive(Clone, Debug)]
pub struct VarInfo {
  pub name: Ident,
  pub kind: Kind,
  pub ty: JType,
  pub field: Option<FieldInfo>,
  /// For each variable, compute the maximum number of `pre` that can possibly happen.
  /// This is useful to compute the size of the stream.
  /// For example: `pre pre x` gives `[x: 2]`.
  pub stream_bound: usize,
}

impl VarInfo {
  fn new(name: Ident, kind: Kind, ty: JType, field: Option<FieldInfo>) -> Self {
    VarInfo {
      name: name,
      kind: kind,
      ty: ty,
      field: field,
      stream_bound: 0
    }
  }

  pub fn local(name: Ident, kind: Kind, ty: JType) -> Self {
    VarInfo::new(name, kind, ty, None)
  }

  pub fn field(name: Ident, kind: Kind, ty: JType,
    visibility: JVisibility, is_ref: Option<Span>) -> Self
  {
    VarInfo::new(name, kind, ty, Some(FieldInfo::new(visibility, is_ref)))
  }

  pub fn mod_name(&self) -> Ident {
    self.ty.name.clone()
  }

  pub fn is_field(&self) -> bool {
    self.field.is_some()
  }

  pub fn is_ref(&self) -> bool {
    match &self.field {
      &Some(ref field) => field.is_ref.is_some(),
      &None => false
    }
  }

  pub fn is_host(&self) -> bool {
    self.kind == Kind::Host
  }
}

#[derive(Clone, Debug)]
pub struct FieldInfo {
  pub visibility: JVisibility,
  pub is_ref: Option<Span>,
}

impl FieldInfo {
  pub fn new(visibility: JVisibility, is_ref: Option<Span>) -> Self {
    FieldInfo {
      visibility: visibility,
      is_ref: is_ref,
    }
  }
}

#[derive(Clone, Debug)]
pub struct ModuleInfo {
  pub name: Ident,
  /// Contains the position and UID of the `ref` variables of this module.
  pub constructor: Vec<(usize, usize)>,
  pub cons_len: usize,
}

impl ModuleInfo {
  pub fn new(name: Ident) -> Self {
    ModuleInfo {
      name: name,
      constructor: vec![],
      cons_len: 0
    }
  }

  pub fn has_refs(&self) -> bool {
    !self.constructor.is_empty()
  }
}

impl Context {
  pub fn new(ast: JCrate) -> Self {
    Context {
      ast: ast,
      vars: vec![VarInfo::local(Ident::gen("<error-var>"), Kind::example(), JType::example())],
      modules: vec![]
    }
  }

  pub fn dummy_ident(&self) -> Ident{
    self.vars[0].name.clone()
  }

  pub fn clone_ast(&self) -> JCrate {
    self.ast.clone()
  }

  pub fn replace_ast(&mut self, ast: JCrate) {
    self.ast = ast;
  }

  fn alloc_var(&mut self, binding: &mut Binding, var_info: VarInfo) -> usize {
    let idx = self.vars.len();
    self.vars.push(var_info);
    binding.uid = idx;
    idx
  }

  pub fn alloc_local(&mut self, binding: &mut Binding) -> usize {
    let info = VarInfo::local(binding.name.clone(), binding.kind, binding.ty.clone());
    self.alloc_var(binding, info)
  }

  pub fn alloc_field(&mut self, field: &mut ModuleField) -> usize {
    let info = VarInfo::field(field.binding.name.clone(),
      field.binding.kind, field.binding.ty.clone(), field.visibility, field.is_ref);
    self.alloc_var(&mut field.binding, info)
  }

  pub fn var_by_uid(&self, uid: usize) -> VarInfo {
    assert!(self.vars.len() > uid, "var_by_uid: Variable not declared.");
    self.vars[uid].clone()
  }

  pub fn var_by_uid_mut<'b>(&'b mut self, uid: usize) -> &'b mut VarInfo {
    assert!(self.vars.len() > uid, "var_by_uid_mut: Variable not declared.");
    &mut self.vars[uid]
  }

  pub fn alloc_module(&mut self, name: Ident) {
    self.modules.push(ModuleInfo::new(name));
  }

  pub fn module_by_name_mut<'b>(&'b mut self, name: Ident) -> &'b mut ModuleInfo {
    self.modules.iter_mut()
      .find(|m| m.name == name)
      .expect("module_by_name_mut: Module not declared.")
  }

  pub fn module_by_name(&self, name: Ident) -> ModuleInfo {
    self.modules.iter()
      .find(|m| m.name == name)
      .cloned()
      .expect("module_by_name: Module not declared.")
  }

  // From a process call, we retrieve its module and definition.
  // It can be used to follow to the call to a process, in contrast to `walk_proc_call` which does not.
  pub fn find_proc_from_call(&self, current_mod: Ident, process: Ident,
    var: Option<Variable>) -> (Ident, Process)
  {
    let bug_msg =
      &format!("[BUG] Verification that processes and modules exist should be done before calling `follow_proc_call`. ({}.{})", current_mod, process);
      let mod_name =
        match var {
          None => current_mod.clone(),
          Some(var) => self.var_by_uid(var.last_uid()).mod_name()
        };
      let module = self.ast.find_mod_by_name(&mod_name).expect(bug_msg);
      (mod_name, module.find_process_by_name(&process).expect(bug_msg))
  }
}
