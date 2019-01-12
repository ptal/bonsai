// Copyright 2019 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// It populates the store of ProcessInfo in `context`, it is used in the backend to generate the code of processes.
/// This class is based on the code of `initialization.rs`.

use context::*;
use session::*;
use std::collections::HashMap;

pub fn collect_module_in_proc(session: Session, context: Context) -> Env<Context> {
  let proc_module = CollectModuleInProc::new(session, context);
  proc_module.collect()
}

struct CollectModuleInProc {
  session: Session,
  context: Context,
  current_proc: ProcessInfo,
}

impl CollectModuleInProc {
  pub fn new(session: Session, context: Context) -> Self {
    let err = Ident::gen("err");
    CollectModuleInProc {
      session: session,
      context: context,
      current_proc: ProcessInfo::new(ProcessUID::new(err.clone(), err))
    }
  }

  fn collect(mut self) -> Env<Context> {
    let bcrate_clone = self.context.clone_ast();
    self.visit_crate(bcrate_clone);
    Env::value(self.session, self.context)
  }

  fn module_local_var(&mut self, binding: &Binding) {
    match binding.expr.clone() {
      Some(expr) => self.module_refs_initializer(binding, expr),
      None => unreachable!("every module variable must have an initializer.")
    }
  }

  fn is_module_ref(&self, binding: &Binding) -> bool {
    self.context.module_by_name(binding.ty.name.clone()).has_refs()
  }

  fn module_refs_initializer(&mut self, binding: &Binding, expr: Expr) {
    if self.is_module_ref(binding) {
      match expr.node {
        ExprKind::NewInstance(new_instance) => {
          self.module_initialization_list(binding, new_instance.ty, new_instance.args);
        }
        _ => unreachable!("every module variable must be initialized with a new statement.")
      }
    }
  }

  fn module_initialization_list(&mut self, binding: &Binding, ty: JType, args: Vec<Expr>)
  {
    let mod_info = self.context.module_by_name(ty.name);
    let mut instantiated_refs = HashMap::new();
    for (pos, uid) in mod_info.constructor {
      let (uid, var) = self.ref_instantiation(&args[pos], uid);
      instantiated_refs.insert(uid, var);
    }
    self.current_proc.push_local_module(binding.uid, instantiated_refs);
  }

  fn ref_instantiation(&mut self, expr: &Expr, uid: usize) -> (usize, Variable) {
    match expr.node.clone() {
      ExprKind::Var(var) => (uid, var),
      _ => unreachable!("all ref in constructor must be initialized with variables.")
    }
  }
}

impl Visitor<JClass> for CollectModuleInProc
{
  fn visit_module(&mut self, module: JModule) {
    self.current_proc.uid.module = module.mod_name();
    walk_processes(self, module.processes);
  }

  fn visit_process(&mut self, process: Process) {
    self.current_proc.uid.process = process.name;
    self.current_proc.local_module_vars.clear();
    self.visit_stmt(process.body);
    self.context.alloc_process(self.current_proc.clone());
  }

  fn visit_binding(&mut self, binding: Binding) {
    if binding.is_module() {
      self.module_local_var(&binding);
    }
  }
}
