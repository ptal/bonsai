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

use ast::*;
use partial::*;

pub fn analyse_bonsai(ast: Program) -> Partial<Module> {
  let mut module = Module {
    header: ast.header,
    class_name: ast.class_name,
    processes: vec![],
    java_methods: vec![]
  };

  let mut body = vec![];
  let mut executable_proc = None;
  for item in ast.items {
    match item {
      Item::Statement(Stmt::Let(decl)) => body.push(Stmt::Let(decl)),
      Item::Statement(stmt) => panic!(
        format!("The following statement cannot appear at top-level: {:?}", stmt)),
      Item::Proc(mut process) => {
        if process.name == "execute" {
          executable_proc = Some(process);
        }
        else {
          process.body = fold_stmt(process.body);
          module.processes.push(process);
        }
      }
      Item::JavaMethod(decl) => module.java_methods.push(decl)
    }
  }
  let mut exec_proc = executable_proc.expect(
    "Missing process `execute`. It is the entry point of the reactive module.");
  body.push(exec_proc.body);
  exec_proc.body = fold_var_sequence(body);
  module.processes.insert(0, exec_proc);
  Partial::Value(module)
}

fn fold_stmt(stmt: Stmt) -> Stmt {
  use ast::Stmt::*;
  match stmt {
    Seq(branches) => fold_var_sequence(branches),
    Par(branches) => Par(fold_stmts(branches)),
    Space(branches) => Space(fold_stmts(branches)),
    When(entailment, body) => When(entailment, Box::new(fold_stmt(*body))),
    Trap(name, body) => Trap(name, Box::new(fold_stmt(*body))),
    Loop(body) => Loop(Box::new(fold_stmt(*body))),
    Let(mut decl) => {
      decl.body = Box::new(fold_stmt(*decl.body));
      Let(decl)
    },
    LetInStore(mut decl) => {
      decl.body = Box::new(fold_stmt(*decl.body));
      LetInStore(decl)
    },
    x => x
  }
}

fn fold_stmts(stmts: Vec<Stmt>) -> Vec<Stmt> {
  stmts.into_iter().map(|stmt| fold_stmt(stmt)).collect()
}

// This method is used to transform:
// Vec[stmt1, LetDecl(..), LetDecl(..), stmt, stmt2] into
// Seq(Vec[stmt1, LetDecl(LetDecl(Seq(stmt2, stmt3))])
fn fold_var_sequence(mut stmts: Vec<Stmt>) -> Stmt {
  use ast::Stmt::*;
  stmts = fold_var(stmts);
  if stmts.len() == 1 {
    fold_stmt(stmts.pop().unwrap())
  }
  else {
    Seq(stmts)
  }
}

fn fold_var(mut stmts: Vec<Stmt>) -> Vec<Stmt> {
  use ast::Stmt::*;
  if stmts.len() == 1 {
    stmts
  }
  else {
    match stmts.remove(0) {
      Let(mut decl) => {
        decl.body = Box::new(fold_var_sequence(stmts));
        vec![Let(decl)]
      },
      LetInStore(mut decl) => {
        decl.body = Box::new(fold_var_sequence(stmts));
        vec![LetInStore(decl)]
      },
      mut front => {
        front = fold_stmt(front);
        let mut rest = fold_var(stmts);
        rest.insert(0, front);
        rest
      }
    }
  }
}
