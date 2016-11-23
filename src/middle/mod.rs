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

use jast::*;
use partial::*;

pub fn analyse_bonsai(ast: Program) -> Partial<JModule> {
  let mut module = Module {
    attributes: vec![],
    processes: vec![],
    host: JClass::new(ast.header, ast.class_name)
  };

  let mut executable_proc = None;
  for item in ast.items {
    match item {
      Item::Attribute(attr) => module.attributes.push(attr),
      Item::Proc(mut process) => {
        if process.name == "execute" {
          executable_proc = Some(process);
        }
        else {
          process.body = fold_stmt(process.body);
          module.processes.push(process);
        }
      }
      Item::JavaMethod(decl) => module.host.java_methods.push(decl),
      Item::JavaAttr(decl) => module.host.java_attrs.push(decl),
      Item::JavaConstructor(decl) => module.host.java_constructors.push(decl)
    }
  }
  let mut exec_proc = executable_proc.expect(
    "Missing process `execute`. It is the entry point of the reactive module.");
  exec_proc.body = wrap_body_with_attr(module.attributes.clone(), exec_proc.body);
  module.processes.insert(0, exec_proc);
  Partial::Value(module)
}

fn wrap_body_with_attr(attrs: Vec<AttributeDecl>, body: Stmt) -> Stmt {
  let mut channel_attrs = vec![];
  let mut mod_attrs = vec![];
  for attr in attrs {
    if attr.is_channel {
      channel_attrs.push(attr.var);
    }
    else {
      mod_attrs.push(attr.var);
    }
  }

  let mut stmts: Vec<_> = mod_attrs.into_iter()
    .map(|attr| Stmt::Let(attr))
    .collect();

  let mut seq_branches: Vec<_> = channel_attrs.into_iter()
    .filter(|attr| !attr.expr.is_bottom())
    .map(|attr| Stmt::Tell(Var::simple(attr.var), attr.expr))
    .collect();
  seq_branches.push(body);
  stmts.push(Stmt::Seq(seq_branches));
  fold_var_sequence(stmts)
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
