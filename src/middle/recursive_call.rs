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

/// Analyse processes that are called recursively.
/// Recursion is forbidden in spacetime because an instant must be statically bounded in time.
/// On the other hand, we do not allow recursion across instant yet to keep the causality analysis simple.

/// In addition, it sets the entry points UID (of the form `module.proc`) in `context` such that:
///   1. They are not called from another process.
///   2. They are not processes from libraries.

use context::*;
use session::*;
use std::collections::HashSet;

pub fn recursive_call(session: Session, context: Context) -> Env<Context> {
  let recursive_call = RecursiveCall::new(session, context);
  recursive_call.analyse()
}

/// `recursion_path` detects a recursive path.
/// `process_visited` avoids reporting an error twice on a same recursive cycle.
struct RecursiveCall {
  session: Session,
  context: Context,
  entry_points: HashSet<ProcessUID>,
  recursion_path: Vec<ProcessUID>,
  process_visited: Vec<ProcessUID>,
  current_module: Ident
}

impl RecursiveCall {
  pub fn new(session: Session, context: Context) -> Self {
    let dummy_ident = context.dummy_ident();
    RecursiveCall {
      session,
      context,
      entry_points: HashSet::new(),
      recursion_path: vec![],
      process_visited: vec![],
      current_module: dummy_ident,
    }
  }

  fn analyse(mut self) -> Env<Context> {
    let bcrate_clone = self.context.clone_ast();
    self.visit_crate(bcrate_clone);
    self.check_entry_points();
    let entry_points: Vec<_> = self.entry_points.into_iter().collect();
    self.context.set_entry_points(entry_points);
    if self.session.has_errors() {
      Env::fake(self.session, self.context)
    } else {
      Env::value(self.session, self.context)
    }
  }

  fn check_entry_points(&self) {
    for uid in self.entry_points.iter().cloned() {
      let process = self.context.find_proc(uid);
      if process.visibility == JVisibility::Private {
        self.warn_private_entry_point(process);
      }
    }
  }

  fn warn_private_entry_point(&self, process: Process) {
    self.session.struct_span_warn_with_code(process.name.span,
      "private process never called.",
      "W0001")
    .help(&"This process is private but never called.\n\
            Solution: Make the process `public` or delete this process.")
    .emit();
  }

  fn err_forbid_recursive_call(&mut self, process: Process) {
    self.session.struct_span_err_with_code(process.name.span,
      "forbidden recursive process call.",
      "E0030")
    .help(&format!(
           "Processes cannot be called recursively due to strong static analysis ensuring determinism of the computation.\n\
            Detected cycle: {}\n\
            Solution: Rewrite your recursive program into an iterative version by using the statement `loop` and the `world_line` variables.",
            self.display_path_cycle()))
    .emit();
  }

  fn display_path_cycle(&self) -> String {
    let mut path_desc = String::new();
    for process in self.recursion_path.iter() {
      path_desc.extend(format!("{} -> ", process).chars());
    }
    path_desc.extend(format!("{}", self.recursion_path[0]).chars());
    path_desc
  }

  fn is_rec(&self, uid: &ProcessUID) -> bool {
    self.recursion_path.iter().any(|p| p == uid)
  }

  fn already_visited(&self, uid: &ProcessUID) -> bool {
    self.process_visited.iter().any(|p| p == uid)
  }

  fn process_uid(&self, process_name: &Ident) -> ProcessUID {
    ProcessUID::new(self.current_module.clone(), process_name.clone())
  }

  fn current_mod_is_lib(&self) -> bool {
    self.context.ast.find_mod_by_name(&self.current_module).unwrap().file.is_lib()
  }

  /// Entry points are added only for processes that are not in a library module, and if they have not been explored before.
  /// This latest condition is important in case a process is called before its declaration is visited (without this condition, we would consider it an entry point).
  fn insert_entry_point(&mut self, process: &Process) {
    let uid = self.process_uid(&process.name);
    if !self.current_mod_is_lib() && !self.already_visited(&uid) {
      self.entry_points.insert(uid);
    }
  }

  fn remove_entry_point(&mut self, uid: &ProcessUID) {
    self.entry_points.remove(uid);
  }
}

impl Visitor<JClass> for RecursiveCall
{
  /// Every process is a potential entry point.
  /// Processes are removed in `visit_proc_call` if they are called.
  fn visit_module(&mut self, module: JModule) {
    let old = self.current_module.clone();
    self.current_module = module.mod_name();
    for process in module.processes {
      self.insert_entry_point(&process);
      self.visit_process(process);
    }
    self.current_module = old;
  }

  fn visit_process(&mut self, process: Process) {
    let uid = self.process_uid(&process.name);
    if self.is_rec(&uid) {
      self.err_forbid_recursive_call(process);
    }
    else {
      if !self.already_visited(&uid) {
        self.process_visited.push(uid.clone());
        self.recursion_path.push(uid.clone());
        self.visit_stmt(process.body);
        self.recursion_path.pop();
      }
    }
  }

  fn visit_proc_call(&mut self, var: Option<Variable>, process: Ident, _args: Vec<Variable>) {
    let (uid, process) = self.context.find_proc_from_call(self.current_module.clone(), process, var);
    self.remove_entry_point(&uid);
    let old = self.current_module.clone();
    self.current_module = uid.module;
    self.visit_process(process);
    self.current_module = old;
  }
}
