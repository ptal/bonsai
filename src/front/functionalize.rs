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

/// The declaration of variable in Java is a statement and not an expression. We perform this transformation as follows:
/// `single_time Type var = expr; <code-following>` into
/// `single_time Type var = expr in <code-following>`.
/// The code following the variable declaration is lifted inside the structure of the let statement.
/// It replaces the `nothing` statement by the statement following inside the AST.

use ast::*;

pub fn let_lifting(mut ast: Program) -> Program {
  ast.items = ast.items.into_iter()
    .map(lift_item)
    .collect();
  ast
}

fn lift_item(item: Item) -> Item {
  if let Item::Proc(mut process) = item {
    process.body = lift_stmt(process.body);
    Item::Proc(process)
  }
  else { item }
}

fn lift_stmt(stmt: Stmt) -> Stmt {
  use ast::StmtKind::*;
  let node = match stmt.node {
    Seq(branches) => lift_let_sequence(branches).node,
    OrPar(branches) => OrPar(lift_stmts(branches)),
    AndPar(branches) => AndPar(lift_stmts(branches)),
    Space(branch) => Space(Box::new(lift_stmt(*branch))),
    When(condition, then_branch, else_branch) =>
      When(condition, Box::new(lift_stmt(*then_branch)), Box::new(lift_stmt(*else_branch))),
    Suspend(condition, body) => Suspend(condition, Box::new(lift_stmt(*body))),
    Abort(condition, body) => Abort(condition, Box::new(lift_stmt(*body))),
    Loop(body) => Loop(Box::new(lift_stmt(*body))),
    Let(mut decl) => {
      decl.body = Box::new(lift_stmt(*decl.body));
      Let(decl)
    },
    x => x
  };
  Stmt::new(stmt.span, node)
}

fn lift_stmts(stmts: Vec<Stmt>) -> Vec<Stmt> {
  stmts.into_iter().map(|stmt| lift_stmt(stmt)).collect()
}

pub fn lift_let_sequence(mut stmts: Vec<Stmt>) -> Stmt {
  stmts = lift_let(stmts);
  if stmts.len() == 1 {
    lift_stmt(stmts.pop().unwrap())
  }
  else {
    Stmt::seq(stmts)
  }
}

fn lift_let(mut stmts: Vec<Stmt>) -> Vec<Stmt> {
  use ast::StmtKind::*;
  if stmts.len() == 1 {
    stmts
  }
  else {
    // If the first statement of the list is a let-decl, then lift the rest of the list in the let-decl.
    match stmts.remove(0) {
      Stmt { node: Let(ref decl), span: _ } if decl.body.is_nothing() => {
        let mut decl = decl.clone();
        decl.body = Box::new(lift_let_sequence(stmts));
        vec![Stmt::new(decl.span, Let(decl))]
      },
      mut front => {
        front = lift_stmt(front);
        let mut rest = lift_let(stmts);
        rest.insert(0, front);
        rest
      }
    }
  }
}
