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
use front::let_lifting::*;
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
      Item::Proc(process) => {
        if process.name == "execute" {
          executable_proc = Some(process);
        }
        else {
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

fn wrap_body_with_attr(attrs: Vec<ModuleAttribute>, body: Stmt) -> Stmt {
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
    .map(|attr| Stmt::Let(LetStmt::placeholder(attr)))
    .collect();

  let mut seq_branches: Vec<_> = channel_attrs.into_iter()
    .filter(|attr| !attr.expr.is_bottom())
    .map(|attr| Stmt::Tell(Var::simple(attr.name), attr.expr))
    .collect();
  seq_branches.push(body);
  stmts.push(Stmt::Seq(seq_branches));
  lift_let_sequence(stmts)
}
