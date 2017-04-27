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

/// Check for undeclared variables.
/// In addition, it computes a unique identifier (UID) for variables local to modules. It does not assign a UID to variable of the form `m.a` or `m.a.b` because the UID for each module's variables is not yet accessible—since being computed. This next step is done in `resolve.rs`.

use context::*;

pub fn undeclared<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  let undeclared = Undeclared::new(context);
  undeclared.analyse()
}

struct Undeclared<'a> {
  context: Context<'a>,
  in_scope_vars: Vec<(Ident, usize)>,
  in_scope_processes: Vec<Ident>,
  visiting_fields: bool,
}

impl<'a> Undeclared<'a> {
  pub fn new(context: Context<'a>) -> Self {
    Undeclared {
      context: context,
      in_scope_vars: Vec::new(),
      in_scope_processes: Vec::new(),
      visiting_fields: false,
    }
  }

  fn session(&'a self) -> &'a Session {
    self.context.session
  }

  fn analyse(mut self) -> Partial<Context<'a>> {
    let mut bcrate_clone = self.context.clone_ast();
    self.visit_crate(&mut bcrate_clone);
    self.context.replace_ast(bcrate_clone);
    if self.session().has_errors() {
      Partial::Fake(self.context)
    } else {
      Partial::Value(self.context)
    }
  }

  fn enter_scope_processes(&mut self, processes: &Vec<Process>) {
    for process in processes {
      self.in_scope_processes.push(process.name.clone());
    }
  }

  fn exit_scope_processes(&mut self, num_processes: usize) {
    for _ in 0..num_processes {
      self.in_scope_processes.pop();
    }
  }

  fn enter_scope(&mut self, binding: &mut Binding) {
    let uid = self.context.alloc_var(binding);
    self.in_scope_vars.push((binding.name.clone(), uid));
  }

  fn exit_scope(&mut self) {
    self.in_scope_vars.pop();
  }

  fn lookup(&self, name: Ident) -> Option<usize> {
    self.in_scope_vars.iter()
      .find(|&&(ref name2, _)| &name == name2)
      .map(|&(_, uid)| uid)
  }

  fn undeclared_var(&mut self, var: &mut Variable) {
    if self.visiting_fields {
      self.err_var_in_field(var);
    }
    else {
      self.undeclared_target(var);
    }
  }

  fn undeclared_target(&mut self, var: &mut Variable) {
    match self.lookup(var.path.target()) {
      Some(uid) => {
        if var.path.is_unary() {
          var.uid = uid;
        }
      }
      None => {
        self.err_undeclared_var(var);
      }
    }
  }

  fn undeclared_process(&mut self, process: Ident) {
    if !self.in_scope_processes.contains(&process) {
      self.err_undeclared_process(process);
    }
  }

  fn err_var_in_field(&mut self, var: &mut Variable) {
    self.session().struct_span_err_with_code(var.span,
      &format!("forbidden occurrence of variable `{}` when declaring a field.", var.path.clone()),
      "E0005")
    .span_label(var.span, &format!("illegal occurrence"))
    .emit();
  }

  fn err_undeclared_var(&mut self, var: &mut Variable) {
    self.session().struct_span_err_with_code(var.span,
      &format!("cannot find variable `{}` in this scope.", var.path.clone()),
      "E0006")
    .span_label(var.span, &format!("undeclared variable"))
    .emit();
  }

  fn err_undeclared_process(&mut self, process: Ident) {
    self.session().struct_span_err_with_code(process.span,
      &format!("cannot find process `{}` in the current module.", process),
      "E0007")
    .span_label(process.span, &format!("undeclared process"))
    .emit();
  }
}

impl<'a> VisitorMut<JClass> for Undeclared<'a>
{
  fn visit_module(&mut self, module: &mut JModule) {
    self.visiting_fields = true;
    for field in &mut module.fields {
      self.enter_scope(&mut field.binding);
      self.visit_field(field);
    }
    self.visiting_fields = false;
    self.enter_scope_processes(&module.processes);
    walk_processes_mut(self, &mut module.processes);
    self.exit_scope_processes(module.processes.len());
    for _ in &module.fields {
      self.exit_scope();
    }
  }

  fn visit_proc_call(&mut self, var: &mut Option<Variable>, process: Ident) {
    match var {
      &mut Some(ref mut var) => self.visit_var(var),
      &mut None => self.undeclared_process(process)
    }
  }

  fn visit_var(&mut self, var: &mut Variable) {
    self.undeclared_var(var);
  }

  fn visit_let(&mut self, let_stmt: &mut LetStmt) {
    self.enter_scope(&mut let_stmt.binding);
    self.visit_binding(&mut let_stmt.binding);
    self.visit_stmt(&mut *(let_stmt.body));
    self.exit_scope();
  }
}

