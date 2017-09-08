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

use std::path::{PathBuf};
use std::process::{Output};
use std::io;

pub struct ExecuteTest<'a>
{
  display: &'a mut Display,
  compile_result: io::Result<Output>,
  execute_result: io::Result<Output>,
  expect: Regex,
  file_path: PathBuf
}

impl<'a> ExecuteTest<'a>
{
  pub fn new(display: &'a mut Display,
    compile_result: io::Result<Output>,
    execute_result: io::Result<Output>,
    expect: Regex,
    file_path: PathBuf) -> Self
  {
    ExecuteTest {
      display, compile_result, execute_result, expect, file_path
    }
  }

  pub fn diagnostic(mut self) {
    if self.diagnose_compilation() {
      self.diagnose_execution();
    }
  }

  fn diagnose_result(&mut self, phase: &str, result: Result<Output, String>) -> Option<Output> {
    match result {
      Result::Ok(output) => Some(output.clone()),
      Result::Err(error) => {
        self.display.io_error(&format!("Failure of the {}.", phase),
          self.file_path.clone(), error);
        None
      }
    }
  }

  fn diagnose_maven_result(&mut self, phase: &str, result: Result<Output, String>) -> Option<Vec<u8>> {
    let result = self.diagnose_result(phase, result);
    if let Some(output) = result {
      if output.status.success() {
        return Some(output.stdout);
      }
      else {
        let file_name = self.file_name();
        self.display.maven_failure(phase, self.file_path.clone(), file_name, output);
      }
    }
    None
  }

  /// io::Error is not cloneable so we transform it into a String now.
  fn clone_result(result: &io::Result<Output>) -> Result<Output, String> {
    match result {
      &Result::Err(ref err) => Result::Err(format!("{}", err)),
      &Result::Ok(ref output) => Result::Ok(output.clone())
    }
  }

  fn diagnose_compilation(&mut self) -> bool {
    let compile_result = Self::clone_result(&self.compile_result);
    self.diagnose_maven_result("compilation", compile_result).is_some()
  }

  fn diagnose_execution(&mut self) {
    let execute_result = Self::clone_result(&self.execute_result);
    let maven_output = self.diagnose_maven_result("execution", execute_result);
    if let Some(raw_stdout) = maven_output {
      match String::from_utf8(raw_stdout) {
        Result::Ok(stdout) => {
          self.compare_output(stdout);
        }
        Result::Err(error) => {
          self.display.io_error(
            &format!("The output of the bonsai code is not in a valid UTF8 encoding."),
            self.file_path.clone(), format!("{}", error));
        }
      }
    }
  }

  fn compare_output(&mut self, output: String) {
    let file_name = self.file_name();
    if self.expect.is_match(&output) {
      self.display.success(file_name);
    }
    else {
      self.display.execution_failure(
        self.file_path.clone(), file_name,
        format!("{}", self.expect.as_str()), output);
    }
  }

  fn file_name(&self) -> String {
    format!("{}", self.file_path.file_name().unwrap().to_str().unwrap())
  }
}
