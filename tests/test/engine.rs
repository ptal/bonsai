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

use libbonsai::session::*;
use libbonsai::driver::*;
use libbonsai::context::*;
use libbonsai::driver::module_file::ModuleFile;

use syntex_syntax::codemap::{CodeMap};
use std::rc::Rc;
use std::cell::RefCell;

use std::path::{PathBuf, Path};
use std::fs::read_dir;

use test::*;
use test::ExpectedResult::*;


pub struct Engine
{
  test_path: PathBuf,
  test_lib: PathBuf,
  display: Display,
  maven: Maven
}

impl Engine
{
  pub fn new(test_path: PathBuf, test_lib: PathBuf) -> Engine
  {
    if !test_path.is_dir() {
      panic!(format!("`{}` is not a valid test directory.", test_path.display()));
    }
    let maven = Maven::new(test_path.clone());
    Engine{
      test_path: test_path,
      test_lib: test_lib,
      display: Display::new(),
      maven: maven
    }
  }

  pub fn run(&mut self)
  {
    let test_path = self.test_path.clone();
    self.display.title("    Bonsai compiler tests suite");
    self.test_directory(format!("Compile and Pass tests."),
      test_path.join(Path::new("compile-pass")), CompileSuccess, false);
    self.test_directory(format!("Compile and Fail tests"),
      test_path.join(Path::new("compile-fail")), CompileFail, false);
    // self.test_directory(format!("Compile and Run tests"),
    //   test_path.join(Path::new("run-pass")), CompileSuccess, true);
    self.display.stats();
    self.display.panic_if_failure();
  }

  fn test_directory(&mut self, start_msg: String, directory: PathBuf,
    expect: ExpectedResult, execute: bool)
  {
    self.display.info(start_msg);
    match read_dir(&directory) {
      Ok(dir_entries) => {
        for entry in dir_entries.map(Result::unwrap).map(|entry| entry.path()) {
          if entry.is_file() {
            self.compile_and_run(entry, expect, execute);
          } else {
            self.display.warn(format!("Entry ignored because it's not a file."));
            self.display.path(entry);
          }
        }
      }
      Err(ref io_err) => {
        self.display.io_error("Can't read directory.", directory, format!("{}", io_err));
      }
    }
  }

  fn compile_and_run(&mut self, filepath: PathBuf, expect: ExpectedResult, execute: bool) {
    println!("{:?}", filepath);
    let obtained_diagnostics = Rc::new(RefCell::new(vec![]));
    let codemap = Rc::new(CodeMap::new());
    let emitter = Box::new(TestEmitter::new(obtained_diagnostics.clone(), codemap.clone()));
    let session = Session::testing_mode(
      filepath.clone(),
      self.maven.source_path(),
      vec![self.test_lib.clone()],
      codemap.clone(), emitter);
    let (session, context) = front_mid_run(session).decompose();
    let session = session.reset_diagnostic();
    let obtained_diagnostics = Rc::try_unwrap(obtained_diagnostics)
      .expect("Could not extract `obtained_diagnostics`.").into_inner();
    let context = {
      let compile_test = CompileTest::new(&mut self.display, context, expect, session.compiler_tests.clone(),
        obtained_diagnostics, filepath.clone(), execute);
      compile_test.diagnostic()
    };
    if let Some(context) = context {
      if execute {
        self.run_file(session, context, filepath);
      }
    }
  }

  /// Given the analysed module (after front and middle phases), we execute the following phases for each test case:
  ///   (1) Compile it (back phase)
  ///   (2) Execute it in a sandbox ("data/test/sandbox")
  ///   (3) Compare the output result with the expected regex result.
  fn run_file(&mut self, mut session: Session, mut context: Context, filepath: PathBuf) {
    self.maven.delete_source_files();
    for test in session.execution_tests.clone() {
      self.maven.delete_source_files();
      session.config.configure_execution_test(&test);
      let env = run_back(session, context)
        .ensure("[Test] Could not generate the Bonsai code.");
      let (s, c) = env.decompose();
      session = s;
      context = c.unwrap();
      let mod_name = ModuleFile::extract_mod_name(filepath.clone()).expect("bonsai file name (run_file)");
      let compile_result = self.maven.compile_sandbox();
      let execute_result = self.maven.execute_sandbox(mod_name);
      let execution_test = ExecuteTest::new(&mut self.display, compile_result,
        execute_result, test.output_regex, filepath.clone());
      execution_test.diagnostic();
      self.maven.delete_source_files();
    }
  }
}
