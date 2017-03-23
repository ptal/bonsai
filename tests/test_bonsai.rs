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

#![feature(plugin, box_syntax, rustc_private)]
#![plugin(oak)]

/// This is a test framework for grammars and inputs that should be accepted or rejected by these grammars.
/// The grammars to test are sub-modules and the inputs are in the directory `data/` at the root of this project. There is one directory per grammar to test with the same name. There is two test mode possible:
/// * Bulk test: Two files are present in the directory and finish with either `.bulk.pass` or `.bulk.fail`. Each line of these files represent one input to test for the grammar considered.
/// * Full test: Two directories are present: `run-pass` and `run-fail` and each files in these directories represent a full input to test against the considered grammar.

extern crate libbonsai;
extern crate syntex_syntax;
extern crate syntex_errors;
extern crate syntex_pos;
extern crate term;
extern crate partial;

use libbonsai::ast::*;
use libbonsai::session::*;
use libbonsai::driver::*;

use partial::*;

use syntex_syntax::codemap::{Pos, CodeMap};
use syntex_errors::DiagnosticBuilder;
use syntex_errors::emitter::Emitter;
use syntex_pos::Span;
use std::rc::Rc;

use std::path::{PathBuf, Path};
use std::fs::read_dir;
use std::io;

use term::*;
use ExpectedResult::*;

#[test]
fn test_data_directory()
{
  let data_path = Path::new("data/");
  if !data_path.is_dir() {
    panic!(format!("`{}` is not a valid data directory.", data_path.display()));
  }
  let mut test_path = PathBuf::new();
  test_path.push(data_path);
  test_path.push(Path::new("test"));
  let mut test_engine = TestEngine::new(test_path);
  test_engine.run();
}

#[derive(Clone, Copy)]
enum ExpectedResult {
  CompileSuccess,
  CompileFail
}

struct TestEngine
{
  test_path: PathBuf,
  display: TestDisplay
}

impl TestEngine
{
  fn new(test_path: PathBuf) -> TestEngine
  {
    if !test_path.is_dir() {
      panic!(format!("`{}` is not a valid test directory.", test_path.display()));
    }
    TestEngine{
      test_path: test_path,
      display: TestDisplay::new()
    }
  }

  fn run(&mut self)
  {
    let test_path = self.test_path.clone();
    self.display.title("    Bonsai compiler tests suite");
    self.test_directory(format!("Compile and Pass tests."),
      test_path.join(Path::new("compile-pass")), CompileSuccess);
    self.test_directory(format!("Compile and Fail tests"),
      test_path.join(Path::new("compile-fail")), CompileFail);
    self.display.stats();
    self.display.panic_if_failure();
  }

  fn test_directory(&mut self, start_msg: String, directory: PathBuf, expect: ExpectedResult) {
    self.display.info(start_msg);
    match read_dir(&directory) {
      Ok(dir_entries) => {
        for entry in dir_entries.map(Result::unwrap).map(|entry| entry.path()) {
          if entry.is_file() {
            self.test_file(entry, expect);
          } else {
            self.display.warn(format!("Entry ignored because it's not a file."));
            self.display.path(entry);
          }
        }
      }
      Err(ref io_err) => {
        self.display.fs_error("Can't read directory.", directory, io_err);
      }
    }
  }

  fn test_file(&mut self, filepath: PathBuf, expect: ExpectedResult) {
    let obtained_diagnostics = Rc::new(vec![]);
    let codemap = Rc::new(CodeMap::new());
    let (result, expected_diagnostics) = {
      let mut session = Session::testing_mode(filepath.clone(), vec![], codemap.clone(),
        Box::new(TestEmitter::new(obtained_diagnostics.clone(), codemap)));
      let result = front_mid_run(&mut session);
      (result, session.expected_diagnostics)
    };
    let test = Test::new(&mut self.display, result, expect, expected_diagnostics,
      Rc::try_unwrap(obtained_diagnostics).unwrap(), filepath);
    test.diagnostic();
  }
}

struct TestEmitter
{
  obtained_diagnostics: Rc<Vec<CompilerDiagnostic>>,
  codemap: Rc<CodeMap>,
}

impl TestEmitter
{
  pub fn new(obtained_diagnostics: Rc<Vec<CompilerDiagnostic>>,
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
    let primary_span: Span = db.span.primary_span().unwrap();
    let loc = self.codemap.lookup_char_pos(primary_span.lo);
    let diagnostic = CompilerDiagnostic::new(
      format!("{}", db.level),
      db.code.clone().unwrap_or(format!("NoCode")),
      loc.line,
      loc.col.to_usize()
    );
    Rc::get_mut(&mut self.obtained_diagnostics).unwrap().push(diagnostic);
  }
}


struct Test<'a>
{
  display: &'a mut TestDisplay,
  result: Partial<JCrate>,
  expect: ExpectedResult,
  expected_diagnostics: Vec<CompilerDiagnostic>,
  obtained_diagnostics: Vec<CompilerDiagnostic>,
  test_path: PathBuf
}

impl<'a> Test<'a>
{
  pub fn new(display: &'a mut TestDisplay,
    result: Partial<JCrate>, expect: ExpectedResult,
    expected_diagnostics: Vec<CompilerDiagnostic>,
    obtained_diagnostics: Vec<CompilerDiagnostic>,
    test_path: PathBuf) -> Self
  {
    Test {
      display: display,
      result: result,
      expect: expect,
      expected_diagnostics: expected_diagnostics,
      obtained_diagnostics: obtained_diagnostics,
      test_path: test_path
    }
  }

  pub fn diagnostic(mut self) {
    let file_name = self.file_name();
    if self.compilation_status(file_name.clone()) {
      self.compare_diagnostics(file_name);
    }
  }

  fn compilation_status(&mut self, file_name: String) -> bool {
    match (self.result.clone(), self.expect) {
      (Partial::Value(_), CompileFail) => {
        self.display.should_fail(self.test_path.clone(), file_name);
        false
      }
      (Partial::Fake(_), CompileSuccess)
    | (Partial::Nothing, CompileSuccess) => {
        self.display.should_succeed(self.test_path.clone(), file_name, &self.obtained_diagnostics);
        false
      }
      _ => true
    }
  }

  fn compare_diagnostics(self, file_name: String) {
    if &self.obtained_diagnostics != &self.expected_diagnostics {
      self.display.diagnostics_failure(self.test_path, file_name,
        &self.obtained_diagnostics,
        &self.expected_diagnostics,
      );
    }
    else {
      self.display.success(file_name);
    }
  }

  fn file_name(&self) -> String {
    format!("{}", self.test_path.file_name().unwrap().to_str().unwrap())
  }
}

struct TestDisplay
{
  terminal: Box<StdoutTerminal>,
  num_success: u32,
  num_failure: u32,
  num_system_failure: u32
}

impl TestDisplay
{
  pub fn new() -> TestDisplay {
    TestDisplay{
      terminal: term::stdout().unwrap(),
      num_success: 0,
      num_failure: 0,
      num_system_failure: 0
    }
  }

  pub fn title(&mut self, msg: &str) {
    self.write_header(color::CYAN, msg);
    self.write_msg("\n\n");
  }

  pub fn info(&mut self, msg: String) {
    self.write_line(color::CYAN, "\n[ info ] ", msg);
  }

  pub fn error(&mut self, msg: String) {
    self.write_line(color::RED, "  [ error ] ", msg);
  }

  pub fn path(&mut self, path: PathBuf) {
    self.write_line(color::CYAN, "  [ path ] ",
      format!("{}", path.display()));
  }

  pub fn stats(&mut self) {
    let system_failure_plural = if self.num_system_failure > 1 { "s" } else { "" };
    let msg = format!("{} passed, {} failed, {} system failure{}.",
        self.num_success, self.num_failure, self.num_system_failure,
        system_failure_plural);
    self.write_line(color::BLUE, "\n\n[ stats ] ", msg);
  }

  pub fn panic_if_failure(&self) {
    if self.num_failure > 0 || self.num_system_failure > 0 {
      panic!("");
    }
  }

  fn failure(&mut self, path: PathBuf, test_name: String)
  {
    self.num_failure += 1;
    self.write_line(color::RED, "[ failed ] ", test_name);
    self.path(path);
  }

  pub fn diagnostics_failure(&mut self, path: PathBuf, test_name: String,
    obtained_diagnostics: &Vec<CompilerDiagnostic>,
    expected_diagnostics: &Vec<CompilerDiagnostic>)
  {
    self.failure(path, test_name);
    self.obtained(obtained_diagnostics);
    self.expected(expected_diagnostics);
  }

  pub fn should_fail(&mut self, path: PathBuf, test_name: String)
  {
    self.failure(path, test_name);
    self.error(format!("Compilation should have failed but succeeded."));
  }

  pub fn should_succeed(&mut self, path: PathBuf, test_name: String,
    obtained_diagnostics: &Vec<CompilerDiagnostic>)
  {
    self.failure(path, test_name);
    self.error(format!("Compilation should have succeeded but failed."));
    self.write_diagnostics(color::RED, "  [ obtained ] ", obtained_diagnostics);
  }

  fn obtained(&mut self, diagnostics: &Vec<CompilerDiagnostic>) {
    self.write_diagnostics(color::CYAN, "  [ obtained ] ", diagnostics);
  }

  fn expected(&mut self, diagnostics: &Vec<CompilerDiagnostic>) {
    self.write_diagnostics(color::CYAN, "  [ expected ] ", diagnostics);
  }

  fn write_diagnostics(&mut self, color: color::Color, header: &str,
   diagnostics: &Vec<CompilerDiagnostic>)
  {
    if diagnostics.len() == 0 {
      self.full_success(color, header);
    }
    else if diagnostics.len() == 1 {
      self.write_line(color, header, format!("{}", diagnostics[0]));
    }
    else {
      self.write_line(color, header, format!("The following errors:"));
      for diagnostic in diagnostics {
        self.write_msg(&format!("    {}\n", diagnostic));
      }
    }
  }

  fn full_success(&mut self, color: color::Color, header: &str) {
    self.write_line(color, header, format!("No diagnostic emitted by the compiler."));
  }

  pub fn success(&mut self, test_name: String) {
    self.num_success += 1;
    self.write_line(color::GREEN, "[ passed ] ", test_name);
  }

  pub fn warn(&mut self, msg: String) {
    self.write_line(color::YELLOW, "  [ warning ] ", msg);
  }

  pub fn fs_error(&mut self, msg: &str, path: PathBuf, io_err: &io::Error) {
    self.system_failure(format!("{}", msg));
    self.path(path);
    self.error(format!("{}", io_err));
  }

  pub fn system_failure(&mut self, msg: String) {
    self.num_system_failure += 1;
    self.write_line(color::RED, "[ system error ] ", msg);
  }

  fn write_line(&mut self, color: color::Color, header: &str, msg: String) {
    self.write_header(color, header);
    self.write_msg(&msg);
    self.write_msg("\n");
  }

  fn write_header(&mut self, color: color::Color, header: &str) {
    self.terminal.fg(color).unwrap();
    self.write_msg(header);
    self.terminal.reset().unwrap();
  }

  fn write_msg(&mut self, msg: &str) {
    (write!(self.terminal, "{}", msg)).unwrap();
  }
}
