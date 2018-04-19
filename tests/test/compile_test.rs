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

use test::*;

use libbonsai::ast::*;
use libbonsai::context::*;

use std::path::{PathBuf};

use ExpectedResult::*;
use ExpectedResult;

pub struct CompileTest<'a>
{
  display: &'a mut Display,
  result: Partial<Context>,
  expect: ExpectedResult,
  expected_diagnostics: Vec<CompilerTest>,
  obtained_diagnostics: Vec<CompilerTest>,
  test_path: PathBuf,
  intermediate_step: bool, // do not print success message because the compilation is just a step of the test.
}

impl<'a> CompileTest<'a>
{
  pub fn new(display: &'a mut Display,
    result: Partial<Context>, expect: ExpectedResult,
    expected_diagnostics: Vec<CompilerTest>,
    obtained_diagnostics: Vec<CompilerTest>,
    test_path: PathBuf,
    intermediate_step: bool) -> Self
  {
    CompileTest {
      display, result, expect, expected_diagnostics,
      obtained_diagnostics, test_path, intermediate_step
    }
  }

  /// Returns the context if the compilation succeeded as expected.
  pub fn diagnostic(mut self) -> Option<Context> {
    let file_name = self.file_name();
    if self.compilation_status(file_name.clone()) {
      self.compare_diagnostics(file_name)
    }
    else {
      None
    }
  }

  pub fn context_to_option(self) -> Option<Context> {
    match self.result {
      Partial::Value(x) => Some(x),
      _ => None
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

  fn compare_diagnostics(self, file_name: String) -> Option<Context> {
    if &self.obtained_diagnostics != &self.expected_diagnostics {
      self.display.diagnostics_failure(self.test_path, file_name,
        &self.obtained_diagnostics,
        &self.expected_diagnostics,
      );
      None
    }
    else {
      if !self.intermediate_step {
        self.display.success(file_name);
      }
      self.context_to_option()
    }
  }

  fn file_name(&self) -> String {
    format!("{}", self.test_path.file_name().unwrap().to_str().unwrap())
  }
}
