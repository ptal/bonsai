// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub struct CodeFormatter {
  indent: usize,
  code: String
}

impl CodeFormatter {
  pub fn new() -> CodeFormatter {
    CodeFormatter {
      indent: 0,
      code: String::new()
    }
  }

  pub fn unwrap(self) -> String {
    self.code
  }

  pub fn indent(&mut self) {
    self.indent += 2;
  }

  pub fn unindent(&mut self) {
    self.indent -= 2;
  }

  pub fn open_block(&mut self) {
    self.push_line("{");
    self.indent();
  }

  pub fn close_block(&mut self) {
    self.unindent();
    self.push_line("}");
  }

  pub fn push(&mut self, code: &str) {
    if self.code.ends_with("\n") {
      self.push_indent();
    }
    self.code += code;
  }

  pub fn push_line(&mut self, code_line: &str) {
    self.push_indent();
    self.terminate_line(code_line);
  }

  pub fn terminate_line(&mut self, code_line: &str) {
    self.code += code_line;
    self.newline();
  }

  pub fn push_java_block(&mut self, code_block: String) {
    let mut lines_iter = code_block.lines();
    self.push_line(lines_iter.next().unwrap());
    for line in lines_iter {
      self.terminate_line(line);
    }
    self.newline();
  }

  pub fn push_block(&mut self, code_block: String) {
    for line in code_block.lines() {
      self.push_line(line);
    }
  }

  pub fn newline(&mut self) {
    self.code += "\n";
  }

  fn push_indent(&mut self) {
    for _ in 0..self.indent {
      self.code.push(' ');
    }
  }
}
