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
}

#[derive(Clone, Debug)]
pub struct VarInfo {
  pub name: Ident,
  pub kind: Kind,
  pub ty: JType,
  // For each variable, compute the maximum number of `pre` that can possibly happen. This is useful to compute the size of the stream. For example: `pre pre x` gives `[x: 2]`.
  pub stream_bound: usize,
}

impl VarInfo {
  pub fn new(name: Ident, kind: Kind, ty: JType) -> Self {
    VarInfo {
      name: name,
      kind: kind,
      ty: ty,
      stream_bound: 0
    }
  }
}

impl<'a> Context<'a> {
  pub fn new(session: &'a Session, ast: JCrate) -> Self {
    Context {
      session: session,
      ast: ast,
      vars: vec![]
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

  pub fn alloc_var(&mut self, binding: &mut Binding) -> usize {
    let idx = self.vars.len();
    self.vars.push(VarInfo::new(binding.name.clone(), binding.kind, binding.ty.clone()));
    binding.uid = idx;
    idx
  }

  pub fn var_by_uid<'b>(&'b self, uid: usize) -> &'b VarInfo {
    assert!(self.vars.len() > uid, "var_by_uid: Variable not declared.");
    &self.vars[uid]
  }

  pub fn var_by_uid_mut<'b>(&'b mut self, uid: usize) -> &'b mut VarInfo {
    assert!(self.vars.len() > uid, "var_by_uid_mut: Variable not declared.");
    &mut self.vars[uid]
  }
}

impl<'a> Deref for Context<'a> {
  type Target = Session;

  fn deref(&self) -> &Session {
    self.session
  }
}
