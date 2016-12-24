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
/// The code following the variable declaration is lifted inside the structure of the let statement. It replaces the `Unknown` statements by the statement following inside the AST.

use jast::*;

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
  use ast::Stmt::*;
  match stmt {
    Seq(branches) => lift_let_sequence(branches),
    Par(branches) => Par(lift_stmts(branches)),
    Space(branches) => Space(lift_stmts(branches)),
    When(entailment, body) => When(entailment, Box::new(lift_stmt(*body))),
    Trap(name, body) => Trap(name, Box::new(lift_stmt(*body))),
    Loop(body) => Loop(Box::new(lift_stmt(*body))),
    Let(mut decl) => {
      decl.body = Box::new(lift_stmt(*decl.body));
      Let(decl)
    },
    x => x
  }
}

fn lift_stmts(stmts: Vec<Stmt>) -> Vec<Stmt> {
  stmts.into_iter().map(|stmt| lift_stmt(stmt)).collect()
}

pub fn lift_let_sequence(mut stmts: Vec<Stmt>) -> Stmt {
  use ast::Stmt::*;
  stmts = lift_let(stmts);
  if stmts.len() == 1 {
    lift_stmt(stmts.pop().unwrap())
  }
  else {
    Seq(stmts)
  }
}

fn lift_let(mut stmts: Vec<Stmt>) -> Vec<Stmt> {
  use ast::Stmt::*;
  if stmts.len() == 1 {
    stmts
  }
  else {
    // If the first statement of the list is a let-decl, then lift the rest of the list in the let-decl.
    match stmts.remove(0) {
      Let(ref decl) if decl.body.is_nothing() => {
        let mut decl = decl.clone();
        decl.body = Box::new(lift_let_sequence(stmts));
        vec![Let(decl)]
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

#[cfg(test)]
mod test
{
  use ast::*;

  #[test]
  fn test_let_lifting() {
    use ast::Stmt::*;
    let let_stmt = Let(LetStmt::imperative(
      LetBinding::Spacetime(LetBindingSpacetime::example())));
    let ast = Seq(vec![
      Stmt::example(),
      let_stmt.clone(),
      Seq(vec![let_stmt.clone(), Stmt::example(), let_stmt.clone()])
    ]);
    let res = super::lift_stmt(ast);
    let expected = Seq(vec![
      Stmt::example(),
      Let(LetStmt::new(LetBinding::Spacetime(LetBindingSpacetime::example()),
        box Let(LetStmt::new(LetBinding::Spacetime(LetBindingSpacetime::example()),
          box Seq(vec![Stmt::example(), let_stmt.clone()])))))
    ]);
    assert_eq!(res, expected);
  }
}

