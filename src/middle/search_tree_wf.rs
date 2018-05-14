// Copyright 2018 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// We verify that the construction of the search tree is well-formed:
///  * A space statement cannot contain another space statement and a prune statement.

use context::*;
use session::*;

pub fn search_tree_wf(session: Session, context: Context) -> Env<Context> {
  let analysis = SearchTreeWellFormedness::new(session, context);
  analysis.analyse()
}

struct SearchTreeWellFormedness {
  session: Session,
  context: Context,
  search_statements: Vec<Span>,
  context_span: Span,
  current_module: Ident
}

impl SearchTreeWellFormedness {
  pub fn new(session: Session, context: Context) -> Self {
    let dummy_ident = context.dummy_ident();
    SearchTreeWellFormedness {
      session: session,
      context: context,
      search_statements: vec![],
      context_span: DUMMY_SP,
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

  fn err_ill_formed_search_tree(&self) {
    let mut db = self.session.struct_span_err_with_code(self.context_span,
      &format!("contains search tree statements (`space` or `prune`)."),
      "E0031");
    for span in self.search_statements.iter() {
      db.span_label(*span, &format!("nested search statement."));
    }
    db.help(&"The process `p` inside `space p end` must not contain `space` and `prune` statements.\n\
            Rational: The process `p` models a branch creating a node, and thus `p` cannot itself create a branch since we are already in a branch.\n\
            Solution: Remove the search tree statements occuring inside the `space` statement.")
    .emit();
  }
}

impl Visitor<JClass> for SearchTreeWellFormedness
{
  fn visit_module(&mut self, module: JModule) {
    let old = self.current_module.clone();
    self.current_module = module.mod_name();
    walk_processes(self, module.processes);
    self.current_module = old;
  }

  fn visit_stmt(&mut self, child: Stmt) {
    let old = self.context_span;
    self.context_span = child.span;
    walk_stmt(self, child);
    self.context_span = old;
  }

  fn visit_space(&mut self, child: Stmt) {
    let old = self.search_statements.clone();
    self.search_statements = vec![];
    self.visit_stmt(child);
    if self.search_statements.len() > 0 {
      self.err_ill_formed_search_tree();
    }
    self.search_statements = old;
    self.search_statements.push(self.context_span);
  }

  fn visit_prune(&mut self) {
    self.search_statements.push(self.context_span);
  }

  fn visit_proc_call(&mut self, var: Option<Variable>, process: Ident, _args: Vec<Variable>) {
    let (mod_name, process) = self.context.find_proc_from_call(self.current_module.clone(), process, var);
    let old_mod = self.current_module.clone();
    self.current_module = mod_name;
    let mut old_search = self.search_statements.clone();
    self.search_statements = vec![];
    self.visit_process(process);
    // Instead of recording all the search statements inside a process call, we only record the call site.
    // It helps to obtain better error messages.
    if self.search_statements.len() > 0 {
      old_search.push(self.context_span);
    }
    self.current_module = old_mod;
    self.search_statements = old_search;
  }
}
