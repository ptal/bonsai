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

extern crate libbonsai;

use test::*;

use libbonsai::ast::*;
use libbonsai::context::*;

use partial::*;

use std::path::{PathBuf};

use ExpectedResult::*;
use ExpectedResult;

pub struct Unit<'a>
{
  display: &'a mut Display,
  result: Partial<Context>,
  expect: ExpectedResult,
  expected_diagnostics: Vec<CompilerDiagnostic>,
  obtained_diagnostics: Vec<CompilerDiagnostic>,
  test_path: PathBuf
}

impl<'a> Unit<'a>
{
  pub fn new(display: &'a mut Display,
    result: Partial<Context>, expect: ExpectedResult,
    expected_diagnostics: Vec<CompilerDiagnostic>,
    obtained_diagnostics: Vec<CompilerDiagnostic>,
    test_path: PathBuf) -> Self
  {
    Unit {
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
    match (&self.result, self.expect) {
      (&Partial::Value(_), CompileFail) => {
        self.display.should_fail(self.test_path.clone(), file_name);
        false
      }
      (&Partial::Fake(_), CompileSuccess)
    | (&Partial::Nothing, CompileSuccess) => {
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
