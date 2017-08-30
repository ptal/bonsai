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

use syntex_syntax::codemap::{CodeMap};
use std::rc::Rc;
use std::cell::RefCell;

use std::path::{PathBuf, Path};
use std::fs::read_dir;
use partial::*;

use test::*;
use test::ExpectedResult::*;


pub struct Engine
{
  test_path: PathBuf,
  test_lib: PathBuf,
  display: Display
}

impl Engine
{
  pub fn new(test_path: PathBuf, test_lib: PathBuf) -> Engine
  {
    if !test_path.is_dir() {
      panic!(format!("`{}` is not a valid test directory.", test_path.display()));
    }
    Engine{
      test_path: test_path,
      test_lib: test_lib,
      display: Display::new()
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
    self.test_directory(format!("Compile and Run tests"),
      test_path.join(Path::new("run-pass")), CompileSuccess, true);
    self.display.stats();
    self.display.panic_if_failure();
  }

  fn test_directory(&mut self, start_msg: String, directory: PathBuf,
    expect: ExpectedResult, run: bool)
  {
    self.display.info(start_msg);
    match read_dir(&directory) {
      Ok(dir_entries) => {
        for entry in dir_entries.map(Result::unwrap).map(|entry| entry.path()) {
          if entry.is_file() {
            self.compile_and_run(entry, expect, run);
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

  fn compile_and_run(&mut self, filepath: PathBuf, expect: ExpectedResult, run: bool) {
    let obtained_diagnostics = Rc::new(RefCell::new(vec![]));
    let codemap = Rc::new(CodeMap::new());
    let emitter = Box::new(TestEmitter::new(obtained_diagnostics.clone(), codemap.clone()));
    let session = Session::testing_mode(filepath.clone(),
      vec![self.test_lib.clone()], codemap.clone(), emitter);
    let (session, context) = front_mid_run(session).decompose();
    let session = session.reset_diagnostic();
    let obtained_diagnostics = Rc::try_unwrap(obtained_diagnostics)
      .expect("Could not extract `obtained_diagnostics`.").into_inner();
    let unit = Unit::new(&mut self.display, context, expect, session.expected_diagnostics,
      obtained_diagnostics, filepath);
    unit.diagnostic();
  }

  fn run_file(&mut self, filepath: PathBuf) {
  }
}
