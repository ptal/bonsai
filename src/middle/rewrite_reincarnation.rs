// Copyright 2018 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// We rewrite `loop p end` into `loop p; p end` to avoid reincarnation problems.
// See the thesis of (Tardieu, 2014), Chapter 7.
// It is the simple rewriting of (Mignard, 1994) possibly exponential in the code size.

use context::*;
use session::*;

pub fn rewrite_reincarnation(session: Session, context: Context) -> Env<Context> {
  let reincarnation = Reincarnation::new(session, context);
  reincarnation.rewrite()
}

struct Reincarnation {
  session: Session,
  context: Context,
  loops: usize,
  rename_mode: bool,
  renamings: Vec<(Ident, Ident)>
}

impl Reincarnation {
  pub fn new(session: Session, context: Context) -> Self {
    Reincarnation { session, context,
      loops: 0, rename_mode: false, renamings: vec![] }
  }

  fn rewrite(mut self) -> Env<Context> {
    let mut bcrate_clone = self.context.clone_ast();
    self.visit_crate(&mut bcrate_clone);
    self.context.replace_ast(bcrate_clone);
    Env::value(self.session, self.context)
  }

  fn rename_decl(&mut self, mut name: Ident) -> Ident {
    let new_name = format!("__reincarn{}_{}", self.loops, name);
    name.value = new_name;
    name
  }

  fn rename(&mut self, var: &mut Variable) {
    for (old, new) in self.renamings.clone() {
      if var.len() == 1 && var.first() == old {
        var.path.fragments[0] = new;
      }
    }
  }
}

impl VisitorMut<JClass> for Reincarnation
{
  fn visit_loop(&mut self, child: &mut Stmt) {
    self.loops += 1;
    self.visit_stmt(child);
    if !self.rename_mode {
      self.rename_mode = true;
      let child_copy = child.clone();
      self.visit_stmt(child);
      let seq = StmtKind::Seq(vec![child.clone(), child_copy]);
      child.node = seq;
      self.rename_mode = false;
    }
    self.loops -= 1;
  }

  fn visit_let(&mut self, let_stmt: &mut LetStmt) {
    if self.rename_mode {
      let old = let_stmt.binding.name.clone();
      let_stmt.binding.name = self.rename_decl(let_stmt.binding.name.clone());
      self.renamings.push((old, let_stmt.binding.name.clone()));
    }
    self.visit_binding(&mut let_stmt.binding);
    self.visit_stmt(&mut *(let_stmt.body));
    if self.rename_mode {
      self.renamings.pop();
    }
  }

  fn visit_var(&mut self, var: &mut Variable) {
    self.rename(var);
  }
}
