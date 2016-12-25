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

use jast::{JParameters,JMethod,JConstructor,JAttribute,JType,JVisibility,JavaCall};
use driver::module_file::ModuleFile;

#[derive(Clone, Debug)]
pub struct Crate<Host> {
  pub modules: Vec<Module<Host>>
}

impl<Host> Crate<Host> where Host: Clone {
  pub fn new() -> Self {
    Crate {
      modules: vec![]
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
  When(EntailmentRel, Box<Stmt>),
  Tell(Var, Expr),
  Pause,
  Trap(String, Box<Stmt>),
  Exit(String),
  Loop(Box<Stmt>),
  ProcCall(String, Vec<Expr>),
  FnCall(Expr),
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
    Stmt::Tell(Var::simple(String::from("x")), Expr::example())
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetBindingBase {
  pub name: String,
  pub ty: JType,
  pub expr: Expr
}

impl LetBindingBase {
  pub fn new(name: String, ty: JType, expr: Expr) -> Self
  {
    LetBindingBase {
      name: name,
      ty: ty,
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
  pub fn new(binding: LetBindingBase) -> Self
  {
    LetBindingModule {
      binding: binding
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    LetBindingModule::new(LetBindingBase::example())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetBindingSpacetime {
  pub binding: LetBindingBase,
  pub spacetime: Spacetime
}

impl LetBindingSpacetime {
  pub fn new(binding: LetBindingBase, sp: Spacetime) -> Self
  {
    LetBindingSpacetime {
      binding: binding,
      spacetime: sp
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    LetBindingSpacetime::new(LetBindingBase::example(), Spacetime::example())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetBindingInStore {
  pub binding: LetBindingBase,
  pub store: String
}

impl LetBindingInStore {
  pub fn new(binding: LetBindingBase, store: String) -> Self {
    LetBindingInStore {
      binding: binding,
      store: store
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntailmentRel {
  pub left: StreamVar,
  pub right: Expr
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Var {
  pub name: String,
  pub args: Vec<Var>
}

impl Var {
  pub fn simple(name: String) -> Self {
    Var {
      name: name,
      args: vec![]
    }
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StreamVar {
  pub name: String,
  pub past: usize,
  pub args: Vec<StreamVar>
}

impl StreamVar {
  pub fn simple(name: String) -> Self {
    StreamVar {
      name: name,
      past: 0,
      args: vec![]
    }
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