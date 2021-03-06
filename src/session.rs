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

#![allow(dead_code)]

use driver::config::*;
use syntex_pos::MultiSpan;
use syntex_errors::DiagnosticBuilder;
use syntex_errors::emitter::{ColorConfig, Emitter};
use syntex_syntax::codemap::{FileMap, CodeMap};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::collections::hash_set::HashSet;
use ast::CompilerTest;
use ast::ExecutionTest;
use partial::*;

pub use syntex_errors::Handler as SpanDiagnostic;

pub type Env<T> = Environment<Session, T>;

pub struct Session {
  pub config: Config,
  pub codemap: Rc<CodeMap>,
  pub span_diagnostic: SpanDiagnostic,
  pub compiler_tests: Vec<CompilerTest>,
  pub execution_tests: Vec<ExecutionTest>,
  // We uniquely identify an error and a warning with its span and error/warning code.
  // This is to avoid registering duplicated errors more than one time.
  // Rational: The body of the loop statement is duplicated which can generate the same error twice. (see `rewrite_reincarnation.rs`).
  errors: HashSet<(MultiSpan, String)>,
  warnings: HashSet<(MultiSpan, String)>,
}

impl Session
{
  fn init(config: Config, codemap: Rc<CodeMap>,
    span_diagnostic: SpanDiagnostic) -> Self
  {
    Session {
      config: config,
      codemap: codemap,
      span_diagnostic: span_diagnostic,
      compiler_tests: vec![],
      execution_tests: vec![],
      errors: HashSet::new(),
      warnings: HashSet::new(),
    }
  }

  pub fn new(config: Config) -> Self {
    let codemap = Rc::new(CodeMap::new());
    let span_diagnostic = SpanDiagnostic::with_tty_emitter(
      ColorConfig::Always, true, false, Some(codemap.clone()));
    Session::init(config, codemap, span_diagnostic)
  }

  pub fn testing_mode(file_to_test: PathBuf, output_dir: PathBuf, libs: Vec<PathBuf>,
    codemap: Rc<CodeMap>, emitter: Box<Emitter>) -> Self
  {
    let span_diagnostic = SpanDiagnostic::with_emitter(
      true, false, emitter);
    Session::init(Config::testing_mode(file_to_test, output_dir, libs),
      codemap, span_diagnostic)
  }

  // `reset_diagnostic` is necessary when testing because `SpanDiagnostic` might encapsulate some references to a shared object.
  // By replacing it, we decrease the reference count.
  pub fn reset_diagnostic(mut self) -> Self {
    self.span_diagnostic = SpanDiagnostic::with_tty_emitter(
      ColorConfig::Always, true, false, Some(self.codemap.clone()));
    self
  }

  pub fn push_compiler_test(&mut self, test: CompilerTest) {
    self.compiler_tests.push(test);
  }

  pub fn push_execution_test(&mut self, test: ExecutionTest) {
    self.execution_tests.push(test);
  }

  pub fn config<'a>(&'a self) -> &'a Config {
    &self.config
  }

  pub fn load_file(&mut self, path: &Path) -> Rc<FileMap> {
    self.codemap.load_file(path).unwrap()
  }

  fn record_error<S: Into<MultiSpan>>(&mut self, sp: S, code: &str) -> bool {
    let code = format!("{}",code);
    // TODO: E0033: To remove when the causality analysis is more precise on the span.
    // TODO: E0014: We should aggregate the errors on reference arguments into a single error.
    code == "E0033" || code == "E0014" || self.errors.insert((sp.into(), code.clone()))
  }

  fn record_warning<S: Into<MultiSpan>>(&mut self, sp: S, code: &str) -> bool {
    self.warnings.insert((sp.into(), format!("{}",code)))
  }

  pub fn diagnostic<'a>(&'a self) -> &'a SpanDiagnostic {
    &self.span_diagnostic
  }

  /// These methods have been extracted from librustc/session/mod.rs
  pub fn struct_span_warn<'a, S: Into<MultiSpan>>(&'a self,
                                                sp: S,
                                                msg: &str)
                                                -> DiagnosticBuilder<'a>  {
    self.diagnostic().struct_span_warn(sp, msg)
  }
  pub fn struct_span_warn_with_code<'a, S: Clone + Into<MultiSpan>>(&'a mut self,
                                                          sp: S,
                                                          msg: &str,
                                                          code: &str)
                                                          -> DiagnosticBuilder<'a>  {
    if self.record_warning(sp.clone(), code) {
      self.diagnostic().struct_span_warn_with_code(sp, msg, code)
    }
    else {
      self.diagnostic().struct_dummy()
    }
  }
  pub fn struct_warn<'a>(&'a self, msg: &str) -> DiagnosticBuilder<'a>  {
    self.diagnostic().struct_warn(msg)
  }
  pub fn struct_span_err<'a, S: Into<MultiSpan>>(&'a self,
                                               sp: S,
                                               msg: &str)
                                               -> DiagnosticBuilder<'a>  {
    self.diagnostic().struct_span_err(sp, msg)
  }
  pub fn struct_span_err_with_code<'a, S: Clone + Into<MultiSpan>>(&'a mut self,
                                                         sp: S,
                                                         msg: &str,
                                                         code: &str)
                                                         -> DiagnosticBuilder<'a>  {
    if self.record_error(sp.clone(), code) {
      self.diagnostic().struct_span_err_with_code(sp, msg, code)
    }
    else {
      self.diagnostic().struct_dummy()
    }
  }
  pub fn struct_err<'a>(&'a self, msg: &str) -> DiagnosticBuilder<'a>  {
    self.diagnostic().struct_err(msg)
  }
  pub fn struct_span_fatal<'a, S: Into<MultiSpan>>(&'a self,
                                                 sp: S,
                                                 msg: &str)
                                                 -> DiagnosticBuilder<'a>  {
    self.diagnostic().struct_span_fatal(sp, msg)
  }
  pub fn struct_span_fatal_with_code<'a, S: Into<MultiSpan>>(&'a self,
                                                           sp: S,
                                                           msg: &str,
                                                           code: &str)
                                                           -> DiagnosticBuilder<'a>  {
    self.diagnostic().struct_span_fatal_with_code(sp, msg, code)
  }
  pub fn struct_fatal<'a>(&'a self, msg: &str) -> DiagnosticBuilder<'a>  {
    self.diagnostic().struct_fatal(msg)
  }

  pub fn span_fatal<S: Into<MultiSpan>>(&self, sp: S, msg: &str) -> ! {
    panic!(self.diagnostic().span_fatal(sp, msg))
  }
  pub fn span_fatal_with_code<S: Into<MultiSpan>>(&self, sp: S, msg: &str, code: &str) -> ! {
    panic!(self.diagnostic().span_fatal_with_code(sp, msg, code))
  }
  pub fn fatal(&self, msg: &str) -> ! {
    panic!(self.diagnostic().fatal(msg))
  }

  pub fn span_err<S: Into<MultiSpan>>(&self, sp: S, msg: &str) {
    self.diagnostic().span_err(sp, msg)
  }
  pub fn span_err_with_code<S: Clone + Into<MultiSpan>>(&mut self, sp: S, msg: &str, code: &str) {
    if self.record_error(sp.clone(), code) {
      self.diagnostic().span_err_with_code(sp, &msg, code);
    }
  }
  pub fn err(&self, msg: &str) {
    self.diagnostic().err(msg)
  }
  pub fn err_count(&self) -> usize {
    self.diagnostic().err_count()
  }
  pub fn has_errors(&self) -> bool {
    self.diagnostic().has_errors()
  }
  pub fn abort_if_errors(&self) {
    self.diagnostic().abort_if_errors();
  }

  pub fn span_warn<S: Into<MultiSpan>>(&self, sp: S, msg: &str) {
    self.diagnostic().span_warn(sp, msg)
  }
  pub fn span_warn_with_code<S: Clone + Into<MultiSpan>>(&mut self, sp: S, msg: &str, code: &str) {
    if self.record_warning(sp.clone(),code) {
      self.diagnostic().span_warn_with_code(sp, msg, code)
    }
  }
  pub fn warn(&self, msg: &str) {
    self.diagnostic().warn(msg)
  }

  pub fn opt_span_warn<S: Into<MultiSpan>>(&self, opt_sp: Option<S>, msg: &str) {
    match opt_sp {
        Some(sp) => self.span_warn(sp, msg),
        None => self.warn(msg),
    }
  }
  /// Delay a span_bug() call until abort_if_errors()
  pub fn delay_span_bug<S: Into<MultiSpan>>(&self, sp: S, msg: &str) {
    self.diagnostic().delay_span_bug(sp, msg)
  }
  pub fn note_without_error(&self, msg: &str) {
    self.diagnostic().note_without_error(msg)
  }
  pub fn span_note_without_error<S: Into<MultiSpan>>(&self, sp: S, msg: &str) {
    self.diagnostic().span_note_without_error(sp, msg)
  }
  pub fn span_unimpl<S: Into<MultiSpan>>(&self, sp: S, msg: &str) -> ! {
    self.diagnostic().span_unimpl(sp, msg)
  }
  pub fn unimpl(&self, msg: &str) -> ! {
    self.diagnostic().unimpl(msg)
  }
}
