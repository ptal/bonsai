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

use std::fmt::{Formatter, Display, Error};
use jast::{JParameters,JMethod,JConstructor,JAttribute,JavaTy,JavaCall};

#[derive(Clone, Debug)]
pub struct Module<Host> {
  pub attributes: Vec<AttributeDecl>,
  pub processes: Vec<Process>,
  pub host: Host
}

#[derive(Clone, Debug)]
pub struct Program {
  pub header: String,
  pub class_name: String,
  pub items: Vec<Item>
}

#[derive(Clone, Debug)]
pub enum Item {
  Attribute(AttributeDecl),
  Proc(Process),
  JavaMethod(JMethod),
  JavaAttr(JAttribute),
  JavaConstructor(JConstructor),
}

#[derive(Clone, Debug)]
pub struct Process {
  pub name: String,
  pub params: JParameters,
  pub body: Stmt
}

impl Process {
  pub fn new(name: String, params: JParameters, body: Stmt) -> Self {
    Process {
      name: name,
      params: params,
      body: body
    }
  }
}

#[derive(Clone, Debug)]
pub enum Stmt {
  Seq(Vec<Stmt>),
  Par(Vec<Stmt>),
  Space(Vec<Stmt>),
  Let(LetDecl),
  LetInStore(LetInStoreDecl),
  When(EntailmentRel, Box<Stmt>),
  Tell(Var, Expr),
  Pause,
  Trap(String, Box<Stmt>),
  Exit(String),
  Loop(Box<Stmt>),
  ProcCall(String, Vec<Expr>),
  FnCall(Expr)
}

#[derive(Clone, Debug)]
pub struct AttributeDecl {
  pub is_channel: bool,
  pub var: LetDecl
}

#[derive(Clone, Debug)]
pub struct LetDecl {
  pub var: String,
  pub var_ty: JavaTy,
  pub spacetime: Spacetime,
  pub expr: Expr,
  pub body: Box<Stmt>
}

#[derive(Clone, Debug)]
pub struct LetInStoreDecl {
  pub location: String,
  pub loc_ty: JavaTy,
  pub store: String,
  pub expr: Expr,
  pub body: Box<Stmt>
}

#[derive(Clone, Debug)]
pub struct EntailmentRel {
  pub left: StreamVar,
  pub right: Expr
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Spacetime {
  SingleSpace,
  SingleTime,
  WorldLine
}

#[derive(Clone, Debug)]
pub enum Expr {
  JavaNew(JavaTy, Vec<Expr>),
  JavaObjectCall(String, Vec<JavaCall>),
  JavaThisCall(JavaCall),
  Number(u64),
  StringLiteral(String),
  Variable(StreamVar),
  Bottom(JavaTy)
}

impl Expr {
  pub fn is_bottom(&self) -> bool {
    match self {
      &Expr::Bottom(_) => true,
      _ => false
    }
  }
}