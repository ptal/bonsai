// Copyright 2018 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// We add the meta statement `drop(x)` to indicate that a variable exit its scope.
/// Rational: It is necessary when generating the guarded commands.
/// We perform this transformation as follows:
/// `single_time Type var = expr in <code-following>` into
/// `single_time Type var = expr in <code-following>; drop(var)`.

use context::*;
use session::*;

pub fn where_is_the_drop(session: Session, context: Context) -> Env<Context> {
  let local_drop = AddLocalDrop::new(session, context);
  local_drop.compute()
}

struct AddLocalDrop {
  session: Session,
  context: Context,
}

impl AddLocalDrop {
  pub fn new(session: Session, context: Context) -> Self {
    AddLocalDrop { session, context }
  }

  fn compute(mut self) -> Env<Context> {
    let mut bcrate_clone = self.context.clone_ast();
    self.visit_crate(&mut bcrate_clone);
    self.context.replace_ast(bcrate_clone);
    Env::value(self.session, self.context)
  }
}

impl VisitorMut<JClass> for AddLocalDrop
{
  fn visit_let(&mut self, let_stmt: &mut LetStmt) {
    use ast::StmtKind::*;
    assert!(let_stmt.binding.uid != 0,
      "AddLocalDrop: We must have resolved the UID before adding the drop statement.");
    let mut path = VarPath::new(DUMMY_SP, vec![let_stmt.binding.name.clone()]);
    path.uids[0] = let_stmt.binding.uid;
    let drop = Stmt::new(DUMMY_SP, LocalDrop(path));
    let mut is_seq = false;
    if let &mut Seq(ref mut branches) = &mut let_stmt.body.node {
      branches.push(drop.clone());
      is_seq = true;
    }
    if !is_seq {
      let_stmt.body.node = Seq(vec![*let_stmt.body.clone(), drop])
    }
  }
}
