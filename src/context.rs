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
use std::fmt::{Display, Error, Formatter};
use std::collections::HashMap;

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

  pub fn is_spacetime(&self) -> bool {
    match self.kind {
      Kind::Spacetime(_) => true,
      _ => false
    }
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ProcessUID {
  pub module: Ident,
  pub process: Ident
}

impl ProcessUID {
  pub fn new(module: Ident, process: Ident) -> Self {
    ProcessUID { module, process }
  }
}

impl Display for ProcessUID {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    formatter.write_fmt(format_args!("{}.{}", self.module, self.process))
  }
}

#[derive(Clone, Debug)]
pub struct LocalModuleVarInfo {
  pub target: usize,
  /// Maps the reference fields to the variable they are instantiated with.
  pub instantiated_refs: HashMap<usize, Variable>
}

impl LocalModuleVarInfo {
  pub fn new(target: usize, instantiated_refs: HashMap<usize, Variable>) -> Self {
    LocalModuleVarInfo { target, instantiated_refs }
  }

  pub fn find_var_by_field_uid(&self, field_uid: usize) -> Variable {
    self.instantiated_refs
      .get(&field_uid)
      .expect(&format!("Local module vars does not contain the field uid {}", field_uid))
      .clone()
  }
}

#[derive(Clone, Debug)]
pub struct ProcessInfo {
  pub uid: ProcessUID,
  pub local_module_vars: Vec<LocalModuleVarInfo>,
}

impl ProcessInfo {
  pub fn new(uid: ProcessUID) -> Self {
    ProcessInfo { uid, local_module_vars: vec![] }
  }

  pub fn push_local_module(&mut self, target: usize, instantiated_refs: HashMap<usize, Variable>) {
    self.local_module_vars.push(LocalModuleVarInfo::new(target, instantiated_refs));
  }
}

pub struct Context {
  pub ast: JCrate,
  /// Indexes of `vars` are referred to as "UIDs", and are contained in the `VarPath` structure.
  /// The index `0` is reserved for external names for which we do not have information.
  /// For example: in `System.out.println`, the class `System` and global variable `out` will have a UID of `0`.
  ///              Similarly for the fields of host objects declared in "bonsai".
  /// Basically, everything we cannot access and that is not part of the "bonsai" world.
  pub vars: Vec<VarInfo>,
  pub modules: Vec<ModuleInfo>,
  pub processes: Vec<ProcessInfo>,
  pub entry_points: Vec<ProcessUID>
}

impl Context {
  pub fn new(ast: JCrate) -> Self {
    Context {
      ast: ast,
      vars: vec![VarInfo::local(Ident::gen("<external-var>"), Kind::Host,
        JType::simple(DUMMY_SP, Ident::gen("<External-Java-type>")))],
      modules: vec![],
      processes: vec![],
      entry_points: vec![]
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

  pub fn set_entry_points(&mut self, entry_points: Vec<ProcessUID>) {
    assert!(self.entry_points.is_empty(), "Context: Entry points have already been set.");
    self.entry_points = entry_points;
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

  pub fn alloc_process(&mut self, proc_info: ProcessInfo) {
    self.processes.push(proc_info);
  }

  pub fn process_by_uid(&self, process_uid: ProcessUID) -> ProcessInfo {
    self.processes.iter()
      .find(|pu| pu.uid == process_uid)
      .expect("process_by_uid_mut: Process not declared.")
      .clone()
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

  pub fn find_proc(&self, uid: ProcessUID) -> Process {
    let bug_msg =
      &format!("[BUG] Verification that processes and modules exist should be done before calling `find_proc`. ({})", uid);
    let module = self.ast.find_mod_by_name(&uid.module).expect(bug_msg);
    module.find_process_by_name(&uid.process).expect(bug_msg)
  }

  // From a process call, we retrieve its module and definition.
  // It can be used to follow to the call to a process, in contrast to `walk_proc_call` which does not.
  pub fn find_proc_from_call(&self, current_mod: Ident, proc_name: Ident,
    var: Option<Variable>) -> (ProcessUID, Process)
  {
    let mod_name =
      match var {
        None => current_mod.clone(),
        Some(var) => self.var_by_uid(var.last_uid()).mod_name()
      };
    let uid = ProcessUID::new(mod_name, proc_name);
    (uid.clone(), self.find_proc(uid))
  }

  /// Check in the imports of the current module `mod_name` if `class_name` is explicitly imported.
  pub fn is_imported(&self, mod_name: &Ident, class_name: &Ident) -> bool {
    let module = self.ast.find_mod_by_name(mod_name)
      .expect(&format!("is_imported: Module `{}` not declared.", mod_name));
    module.host.is_imported(class_name)
  }
}
