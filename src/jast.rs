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
  pub generics: Vec<JType>
}

impl JType {
  pub fn simple(name: String) -> Self {
    JType {
      name: name,
      generics: vec![]
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
