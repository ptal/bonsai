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

use std::path::{PathBuf};
use std::process::{Output};

use term;
use term::color;

pub struct Display
{
  terminal: Box<term::StdoutTerminal>,
  num_success: u32,
  num_failure: u32,
  num_system_failure: u32
}

impl Display
{
  pub fn new() -> Display {
    Display{
      terminal: term::stdout().unwrap(),
      num_success: 0,
      num_failure: 0,
      num_system_failure: 0
    }
  }

  fn write_line(&mut self, color: color::Color, header: &str, msg: String) {
    self.write_header(color, header);
    self.write_msg(&msg);
    self.write_msg("\n");
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
    obtained_diagnostics: &Vec<CompilerTest>,
    expected_diagnostics: &Vec<CompilerTest>)
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
    obtained_diagnostics: &Vec<CompilerTest>)
  {
    self.failure(path, test_name);
    self.error(format!("Compilation should have succeeded but failed."));
    self.write_diagnostics(color::RED, "  [ obtained ] ", obtained_diagnostics);
  }

  fn obtained(&mut self, diagnostics: &Vec<CompilerTest>) {
    self.write_diagnostics(color::CYAN, "  [ obtained ] ", diagnostics);
  }

  fn expected(&mut self, diagnostics: &Vec<CompilerTest>) {
    self.write_diagnostics(color::CYAN, "  [ expected ] ", diagnostics);
  }

  fn write_diagnostics(&mut self, color: color::Color, header: &str,
   diagnostics: &Vec<CompilerTest>)
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

  pub fn io_error(&mut self, msg: &str, path: PathBuf, err: String) {
    self.system_failure(format!("{}", msg));
    self.path(path);
    self.error(err);
  }

  pub fn system_failure(&mut self, msg: String) {
    self.num_system_failure += 1;
    self.write_line(color::RED, "[ system error ] ", msg);
  }

  fn write_header(&mut self, color: color::Color, header: &str) {
    self.terminal.fg(color).unwrap();
    self.write_msg(header);
    self.terminal.reset().unwrap();
  }

  fn write_msg(&mut self, msg: &str) {
    (write!(self.terminal, "{}", msg)).unwrap();
  }

  fn write_maven_output(&mut self, color: color::Color, header: &str, output: Vec<u8>) {
    if output.len() == 0 {
      self.write_line(color, header, format!("Maven did not write on this output stream."));
    }
    else {
      self.write_line(color, header, format!("Maven produced the following output:"));
      match String::from_utf8(output.clone()) {
        Result::Ok(output) => self.write_msg(&output),
        Result::Err(_) => self.write_msg(&format!("{:?}", output))
      }
    }
  }

  pub fn run_success(&mut self, test_name: String, process_name: String) {
    self.success(format!("{}.{}", test_name, process_name))
  }

  fn run_failure(&mut self, path: PathBuf, test_name: String, process_name: String) {
    self.failure(path, format!("{}.{}", test_name, process_name))
  }

  pub fn maven_failure(&mut self, phase: &str, path: PathBuf,
    test_name: String, process_name: String, output: Output)
  {
    self.run_failure(path, test_name, process_name);
    self.error(format!("Maven {} should have succeeded but failed.", phase));
    self.write_maven_output(color::CYAN, "  [ stdout ] ", output.stdout);
    self.write_maven_output(color::CYAN, "  [ stderr ] ", output.stderr);
  }

  pub fn execution_failure(&mut self, path: PathBuf, test_name: String,
    process_name: String, expected: String, obtained: String)
  {
    self.run_failure(path, test_name, process_name);
    self.error(format!("The output does not match the expected regex."));
    self.write_line(color::CYAN, "  [ expected ] ", expected);
    self.write_line(color::CYAN, "  [ obtained ] ", obtained);
  }
}
