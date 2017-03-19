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

use driver::module_file::ModuleFile;
use std::fmt::{Display, Error, Formatter};
use std::collections::HashMap;
pub use syntex_pos::Span;

#[derive(Clone, Debug)]
pub struct Crate<Host> {
  pub modules: Vec<Module<Host>>,
  pub stream_bound: HashMap<String, usize>
}

impl<Host> Crate<Host> where Host: Clone {
  pub fn new() -> Self {
    Crate {
      modules: vec![],
      stream_bound: HashMap::new()
    }
  }

  pub fn find_mod_by_name(&self, name: String) -> Option<Module<Host>> {
    self.modules.iter()
      .find(|m| m.file.mod_name() == name).cloned()
  }
}

#[derive(Clone, Debug)]
pub struct Module<Host> {
  pub attributes: Vec<ModuleAttribute>,
  pub processes: Vec<Process>,
  pub file: ModuleFile,
  pub host: Host
}

impl<Host> Module<Host> {
  pub fn channel_attrs(&self) -> Vec<LetBinding> {
    self.attributes.iter()
      .filter(|a| a.is_channel)
      .cloned()
      .map(|a| a.binding)
      .collect()
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleAttribute {
  pub visibility: JVisibility,
  pub binding: LetBinding,
  pub is_channel: bool,
}

#[derive(Clone, Debug)]
pub struct Program {
  pub header: String,
  pub class_name: String,
  pub items: Vec<Item>
}

#[derive(Clone, Debug)]
pub enum Item {
  Attribute(ModuleAttribute),
  Proc(Process),
  JavaMethod(JMethod),
  JavaAttr(JAttribute),
  JavaConstructor(JConstructor),
}

#[derive(Clone, Debug)]
pub struct Process {
  pub visibility: JVisibility,
  pub name: String,
  pub params: JParameters,
  pub body: Stmt
}

impl Process {
  pub fn new(vis: JVisibility, name: String, params: JParameters, body: Stmt) -> Self {
    Process {
      visibility: vis,
      name: name,
      params: params,
      body: body
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
  Seq(Vec<Stmt>),
  Par(Vec<Stmt>),
  Space(Vec<Stmt>),
  Let(LetStmt),
  When(Condition, Box<Stmt>),
  Tell(StreamVar, Expr),
  Pause(Span),
  Trap(String, Box<Stmt>),
  Exit(String),
  Loop(Box<Stmt>),
  ProcCall(String, Vec<Expr>),
  FnCall(Expr),
  ModuleCall(RunExpr),
  Nothing // This is a facility for parsing, passing from imperative to functional representation. (see let_lifting.rs).
}

impl Stmt {
  pub fn is_nothing(&self) -> bool {
    match self {
      &Stmt::Nothing => true,
      _ => false
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Stmt::Tell(StreamVar::example(), Expr::example())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunExpr {
  pub module_path: VarPath,
  pub process: String
}

impl RunExpr {
  pub fn main(module_path: VarPath) -> Self {
    RunExpr::new(module_path, String::from("execute"))
  }

  pub fn new(module_path: VarPath, process: String) -> Self {
    RunExpr {
      module_path: module_path,
      process: process
    }
  }

  pub fn to_expr(mut self) -> Expr {
    let head_var = self.module_path.properties.remove(0);
    let mut jcalls = self.module_path.to_java_calls();
    jcalls.push(JavaCall::empty_method(self.process));
    Expr::JavaObjectCall(head_var, jcalls)
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStmt {
  pub binding: LetBinding,
  pub body: Box<Stmt>
}

impl LetStmt {
  pub fn new(binding: LetBinding, body: Box<Stmt>) -> Self {
    LetStmt {
      binding: binding,
      body: body
    }
  }

  pub fn imperative(binding: LetBinding) -> Self {
    LetStmt::new(binding, Box::new(Stmt::Nothing))
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LetBinding {
  InStore(LetBindingInStore),
  Spacetime(LetBindingSpacetime),
  Module(LetBindingModule)
}

impl LetBinding {
  pub fn base(&self) -> LetBindingBase {
    use self::LetBinding::*;
    match self.clone() {
      InStore(base) => base.binding,
      Spacetime(base) => base.binding,
      Module(base) => base.binding
    }
  }
  pub fn base_mut<'a>(&'a mut self) -> &'a mut LetBindingBase {
    use self::LetBinding::*;
    match self {
      &mut InStore(ref mut base) => &mut base.binding,
      &mut Spacetime(ref mut base) => &mut base.binding,
      &mut Module(ref mut base) => &mut base.binding
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetBindingBase {
  pub name: String,
  pub ty: JType,
  pub is_module_attr: bool,
  pub expr: Expr
}

impl LetBindingBase {
  pub fn new(name: String, ty: JType, expr: Expr) -> Self
  {
    LetBindingBase {
      name: name,
      ty: ty,
      is_module_attr: false,
      expr: expr
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    LetBindingBase::new(String::from("<name>"), JType::example(),
      Expr::example())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetBindingModule {
  pub binding: LetBindingBase
}

impl LetBindingModule {
  pub fn new(binding: LetBindingBase) -> Self {
    LetBindingModule {
      binding: binding
    }
  }

  pub fn module_name(&self) -> String {
    self.binding.ty.name.clone()
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    LetBindingModule::new(LetBindingBase::example())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetBindingSpacetime {
  pub binding: LetBindingBase,
  pub spacetime: Spacetime,
  pub is_transient: bool,
}

impl LetBindingSpacetime {
  pub fn new(binding: LetBindingBase, sp: Spacetime, is_transient: bool) -> Self
  {
    let is_transient = if sp == Spacetime::SingleTime { true } else { is_transient };
    LetBindingSpacetime {
      binding: binding,
      spacetime: sp,
      is_transient: is_transient,
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    LetBindingSpacetime::new(LetBindingBase::example(), Spacetime::example(), false)
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetBindingInStore {
  pub binding: LetBindingBase,
  pub store: VarPath
}

impl LetBindingInStore {
  pub fn new(binding: LetBindingBase, store: VarPath) -> Self {
    LetBindingInStore {
      binding: binding,
      store: store
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Condition {
  Entailment(EntailmentRel),
  MetaEntailment(MetaEntailmentRel)
}

impl Condition {
  pub fn unwrap(self) -> EntailmentRel {
    match self {
      Condition::Entailment(rel) => rel,
      Condition::MetaEntailment(meta) => meta.left
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntailmentRel {
  pub left: StreamVar,
  pub right: Expr
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MetaEntailmentRel {
  pub left: EntailmentRel,
  pub right: bool
}

/// A variable path can be `x`, `m.x`, `m.m2.y`,... where `m` and `m2` must be checked to be module.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct VarPath {
  pub properties: Vec<String>
}

impl VarPath {
  pub fn new(properties: Vec<String>) -> Self {
    VarPath {
      properties: properties
    }
  }
  pub fn to_java_calls(&self) -> Vec<JavaCall> {
    self.properties.clone().into_iter()
      .map(|p| JavaCall::attribute(p))
      .collect()
  }

  pub fn name(&self) -> String {
    self.properties.last().unwrap().clone()
  }
}

impl Display for VarPath {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let mut i = 0;
    while i < self.properties.len() - 1 {
      fmt.write_fmt(format_args!("{}.", self.properties[i]))?;
      i += 1;
    }
    fmt.write_str(self.properties[i].as_str())
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StreamVar {
  pub path: VarPath,
  pub past: usize,
  pub args: Vec<StreamVar>
}

impl StreamVar {
  pub fn new(path: VarPath, args: Vec<StreamVar>, past: usize) -> Self {
    StreamVar {
      path: path,
      past: past,
      args: args
    }
  }
  pub fn simple(name: String) -> Self {
    Self::present(VarPath::new(vec![name]), vec![])
  }

  pub fn present(path: VarPath, args: Vec<StreamVar>) -> Self {
    Self::new(path, args, 0)
  }

  pub fn name(&self) -> String {
    self.path.name()
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Self::simple(String::from("x"))
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Spacetime {
  SingleSpace,
  SingleTime,
  WorldLine
}

impl Spacetime {
  #[allow(dead_code)]
  pub fn example() -> Self {
    Spacetime::SingleSpace
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
  JavaNew(JType, Vec<Expr>),
  JavaObjectCall(String, Vec<JavaCall>),
  JavaThisCall(JavaCall),
  Boolean(bool),
  Number(u64),
  StringLiteral(String),
  Variable(StreamVar),
  Bottom(JType)
}

impl Expr {
  pub fn is_bottom(&self) -> bool {
    match self {
      &Expr::Bottom(_) => true,
      _ => false
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Expr::Variable(StreamVar::simple(String::from("<expr>")))
  }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JClass {
  pub header: String,
  pub class_name: String,
  pub java_methods: Vec<JMethod>,
  pub java_attrs: Vec<JAttribute>,
  pub java_constructors: Vec<JConstructor>,
}

impl JClass {
  pub fn new(header: String, class_name: String) -> Self {
    JClass {
      header: header,
      class_name: class_name,
      java_methods: vec![],
      java_attrs: vec![],
      java_constructors: vec![]
    }
  }
}

pub type JCrate = Crate<JClass>;
pub type JModule = Module<JClass>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JMethod {
  pub visibility: JVisibility,
  pub is_static: bool,
  pub return_ty: JType,
  pub name: String,
  pub parameters: JParameters,
  pub body: JavaBlock
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JConstructor {
  pub visibility: JVisibility,
  pub name: String,
  pub parameters: JParameters,
  pub body: JavaBlock
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JAttribute {
  pub visibility: JVisibility,
  pub is_static: bool,
  pub ty: JType,
  pub name: String,
  pub expr: Option<Expr>,
}

pub type JavaBlock = String;
pub type JParameters = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JType {
  pub name: String,
  pub generics: Vec<JType>,
  pub is_array: bool
}

impl JType {
  pub fn simple(name: String) -> Self {
    JType {
      name: name,
      generics: vec![],
      is_array: false
    }
  }

  pub fn example() -> Self {
    JType::simple(String::from("<Java type>"))
  }
}

impl Display for JType
{
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    formatter.write_fmt(format_args!("{}", self.name))?;
    if !self.generics.is_empty() {
      let mut generics_str = String::from("<");
      for generic in &self.generics {
        generics_str.push_str(format!("{}, ", generic).as_str());
      }
      // Remove the extra ", " characters.
      generics_str.pop();
      generics_str.pop();
      formatter.write_fmt(format_args!("{}>", generics_str))?;
    }
    if self.is_array {
      formatter.write_str("[]")?;
    }
    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JVisibility {
  Public,
  Protected,
  Private,
}

impl Display for JVisibility {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    use self::JVisibility::*;
    match self {
      &Public => formatter.write_str("public"),
      &Protected => formatter.write_str("protected"),
      &Private => formatter.write_str("private"),
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaCall {
  pub property: String, // can be an attribute or a method.
  pub is_attribute: bool,
  pub args: Vec<Expr>
}

impl JavaCall {
  pub fn empty_method(name: String) -> Self {
    JavaCall {
      property: name,
      is_attribute: false,
      args: vec![]
    }
  }

  pub fn attribute(name: String) -> Self {
    JavaCall {
      property: name,
      is_attribute: true,
      args: vec![]
    }
  }
}
