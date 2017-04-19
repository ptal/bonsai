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

/// We create a module, ensure that an entry point exists (`execute` method) and move module fields as `let` declarations wrapping the `execute` code.

use ast::*;
use driver::module_file::ModuleFile;
use front::let_lifting::*;
use partial::*;

pub fn functionalize_module(file: ModuleFile, ast: Program) -> Partial<JModule> {
  let mut module = Module {
    fields: vec![],
    processes: vec![],
    file: file,
    host: JClass::new(ast.header, ast.package, ast.imports, ast.class_name, ast.interfaces)
  };

  let mut executable_proc = None;
  for item in ast.items {
    match item {
      Item::Field(field) => module.fields.push(field),
      Item::Proc(process) => {
        if process.name == "execute" {
          executable_proc = Some(process);
        }
        else {
          module.processes.push(process);
        }
      }
      Item::JavaMethod(decl) => module.host.java_methods.push(decl),
      Item::JavaField(decl) => module.host.java_fields.push(decl),
      Item::JavaConstructor(decl) => module.host.java_constructors.push(decl)
    }
  }
  let mut exec_proc = executable_proc.expect(&format!(
    "Missing process `execute` in `{}`. It is the entry point of the reactive module.",
    module.file.mod_name()));
  exec_proc.body = functionalize_attrs(module.fields.clone(), exec_proc.body);
  module.processes.insert(0, exec_proc);
  Partial::Value(module)
}

fn functionalize_attrs(fields: Vec<ModuleField>, body: Stmt) -> Stmt {
  let mut ref_fields = vec![];
  let mut mod_fields = vec![];
  for field in fields {
    if field.is_ref {
      ref_fields.push(field.binding);
    }
    else {
      mod_fields.push(field.binding);
    }
  }

  let mut stmts: Vec<_> = mod_fields.into_iter()
    .map(|binding| Stmt::field(binding))
    .collect();

  let mut seq_branches: Vec<_> = ref_fields.into_iter()
    .filter(|binding| !binding.expr.node.is_bottom())
    .map(|binding| Stmt::new(binding.span,
      StmtKind::Tell(StreamVar::simple(binding.span, binding.name), binding.expr)))
    .collect();
  let body_sp = body.span;
  seq_branches.push(body);
  stmts.push(Stmt::new(body_sp, StmtKind::Seq(seq_branches)));
  lift_let_sequence(stmts)
}
