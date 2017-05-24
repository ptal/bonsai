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
pub use session::*;
pub use visitor::*;
pub use partial::*;
use driver::config::Config;
use std::ops::Deref;

pub struct Context<'a> {
  pub session: &'a Session,
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
  // For each variable, compute the maximum number of `pre` that can possibly happen. This is useful to compute the size of the stream. For example: `pre pre x` gives `[x: 2]`.
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

impl<'a> Context<'a> {
  pub fn new(session: &'a Session, ast: JCrate) -> Self {
    Context {
      session: session,
      ast: ast,
      vars: vec![VarInfo::local(Ident::gen("<error-var>"), Kind::example(), JType::example())],
      modules: vec![]
    }
  }

  pub fn config(&self) -> &'a Config {
    self.session.config()
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

}

impl<'a> Deref for Context<'a> {
  type Target = Session;

  fn deref(&self) -> &Session {
    self.session
  }
}
