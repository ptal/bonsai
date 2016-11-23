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

pub use ast::*;
use std::fmt::{Formatter, Display, Error};

#[derive(Clone, Debug)]
pub struct JClass {
  pub header: String,
  pub class_name: String,
  pub java_methods: Vec<JavaMethodDecl>,
  pub java_attrs: Vec<JavaAttrDecl>,
  pub java_constructors: Vec<JavaConstructorDecl>,
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

pub type JModule = Module<JClass>;


#[derive(Clone, Debug)]
pub struct JavaMethodDecl {
  pub visibility: JavaVisibility,
  pub is_static: bool,
  pub return_ty: JavaTy,
  pub name: String,
  pub parameters: JavaParameters,
  pub body: JavaBlock
}

#[derive(Clone, Debug)]
pub struct JavaConstructorDecl {
  pub visibility: JavaVisibility,
  pub name: String,
  pub parameters: JavaParameters,
  pub body: JavaBlock
}

#[derive(Clone, Debug)]
pub struct JavaAttrDecl {
  pub visibility: JavaVisibility,
  pub is_static: bool,
  pub ty: JavaTy,
  pub name: String,
  pub expr: Option<Expr>,
}

pub type JavaBlock = String;
pub type JavaParameters = String;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum JavaVisibility {
  Public,
  Protected,
  Private,
}

impl Display for JavaVisibility {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    use self::JavaVisibility::*;
    match self {
      &Public => formatter.write_str("public"),
      &Protected => formatter.write_str("protected"),
      &Private => formatter.write_str("private"),
    }
  }
}

#[derive(Clone, Debug)]
pub struct JavaCall {
  pub property: String, // can be an attribute or a method.
  pub is_attribute: bool,
  pub args: Vec<Expr>
}
