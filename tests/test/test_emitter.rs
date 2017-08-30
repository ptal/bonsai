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

use libbonsai::ast::*;

use syntex_syntax::codemap::{Pos, CodeMap};
use syntex_errors::DiagnosticBuilder;
use syntex_errors::emitter::Emitter;
use syntex_pos::Span;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TestEmitter
{
  obtained_diagnostics: Rc<RefCell<Vec<CompilerTest>>>,
  codemap: Rc<CodeMap>,
}

impl TestEmitter
{
  pub fn new(obtained_diagnostics: Rc<RefCell<Vec<CompilerTest>>>,
   codemap: Rc<CodeMap>) -> Self
  {
    TestEmitter {
      obtained_diagnostics: obtained_diagnostics,
      codemap: codemap
    }
  }
}

impl Emitter for TestEmitter
{
  fn emit(&mut self, db: &DiagnosticBuilder) {
    let primary_span: Span = db.span.primary_span()
      .expect("Diagnostic lacks a primary span.");
    let loc = self.codemap.lookup_char_pos(primary_span.lo);
    let diagnostic = CompilerTest::new(
      format!("{}", db.level),
      db.code.clone().unwrap_or(format!("NoCode")),
      loc.line,
      loc.col.to_usize()
    );
    self.obtained_diagnostics
      .try_borrow_mut()
      .expect("Could not mutably borrow `obtained_diagnostics`")
      .push(diagnostic);
  }
}
