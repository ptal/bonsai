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

pub struct Program {
  pub header: String,
  pub class_name: String,
  pub items: Vec<Item>
}

pub enum Item {
  Statement(Stmt),
  Proc(Processus),
  JavaStaticMethod(String, String)
}

pub type Block = Vec<Stmt>;

pub type JavaBlock = String;
pub type JavaParameters = String;

pub struct JavaTy {
  pub name: String,
  pub generics: Vec<JavaTy>
}

impl Display for JavaTy
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
    Ok(())
  }
}

pub struct Processus {
  pub name: String,
  pub params: Vec<String>,
  pub body: Block
}

pub enum Stmt {
  Par(Vec<Block>),
  Space(Vec<Block>),
  Let(LetDecl),
  LetInStore(LetInStoreDecl),
  When(EntailmentRel, Block),
  Pause,
  Trap(String, Block),
  Exit(String),
  Loop(Block),
  FnCall(String, Vec<Expr>),
  Tell(Var, Expr)
}

pub struct LetDecl {
  pub transient: bool,
  pub var: String,
  pub var_ty: Option<JavaTy>,
  pub spacetime: Spacetime,
  pub expr: Expr
}

pub struct LetInStoreDecl {
  pub location: String,
  pub loc_ty: Option<JavaTy>,
  pub store: String,
  pub expr: Expr
}

pub struct EntailmentRel {
  pub left: StreamVar,
  pub right: Expr
}

pub struct Var {
  pub name: String,
  pub args: Vec<Var>
}

pub struct StreamVar {
  pub name: String,
  pub past: usize,
  pub args: Vec<StreamVar>
}

pub enum Spacetime {
  SingleSpace,
  SingleTime,
  WorldLine,
  Location(String)
}

pub enum Expr {
  JavaNew(JavaTy, Vec<Expr>),
  JavaObjectCall(String, Vec<JavaCall>),
  Number(u64),
  StringLiteral(String),
  Variable(StreamVar)
}

pub struct JavaCall {
  pub property: String, // can be an attribute or a method.
  pub args: Vec<Expr>
}
