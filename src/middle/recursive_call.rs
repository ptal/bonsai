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

use context::*;
use session::*;

pub fn recursive_call(session: Session, context: Context) -> Env<Context> {
  let recursive_call = RecursiveCall::new(session, context);
  recursive_call.analyse()
}

/// `recursion_path` detects a recursive path.
/// `process_visited` avoids reporting an error twice on a same recursive cycle.
struct RecursiveCall {
  session: Session,
  context: Context,
  recursion_path: Vec<String>,
  process_visited: Vec<String>,
  current_module: Ident
}

impl RecursiveCall {
  pub fn new(session: Session, context: Context) -> Self {
    let dummy_ident = context.dummy_ident();
    RecursiveCall {
      session: session,
      context: context,
      recursion_path: vec![],
      process_visited: vec![],
      current_module: dummy_ident,
    }
  }

  fn analyse(mut self) -> Env<Context> {
    let bcrate_clone = self.context.clone_ast();
    self.visit_crate(bcrate_clone);
    if self.session.has_errors() {
      Env::fake(self.session, self.context)
    } else {
      Env::value(self.session, self.context)
    }
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

  fn is_rec(&self, uid: &String) -> bool {
    self.recursion_path.iter().any(|p| p == uid)
  }

  fn already_visited(&self, uid: &String) -> bool {
    self.process_visited.iter().any(|p| p == uid)
  }

  fn process_uid(&self, process: &Ident) -> String {
    format!("{}.{}", self.current_module, process)
  }
}

impl Visitor<JClass> for RecursiveCall
{
  fn visit_module(&mut self, module: JModule) {
    let old = self.current_module.clone();
    self.current_module = module.mod_name();
    walk_processes(self, module.processes);
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
    let (mod_name, process) = self.context.find_proc_from_call(self.current_module.clone(), process, var);
    let old = self.current_module.clone();
    self.current_module = mod_name;
    self.visit_process(process);
    self.current_module = old;
  }
}
