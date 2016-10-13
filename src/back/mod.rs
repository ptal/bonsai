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

use ast::*;
use partial::*;

pub fn generate_chococubes(ast: Program) -> Partial<String> {
  let mut gen = CodeGenerator::new();
  gen.push_block(ast.header);
  gen.push_line(&format!("public class {}", ast.class_name));
  gen.open_block();
  generate_items(&mut gen, ast.items);
  gen.close_block();

  Partial::Value(gen.code)
}

fn generate_items(gen: &mut CodeGenerator, items: Vec<Item>) {
  for item in items {
    match item {
      Item::Statement(stmt) => generate_statement(gen, stmt),
      Item::Proc(process) => generate_process(gen, process),
      Item::JavaStaticMethod(_, method) => gen.push_java_method(method)
    }
  }
}

fn generate_statement(gen: &mut CodeGenerator, stmt: Stmt) {
}

fn generate_process(gen: &mut CodeGenerator, process: Process) {

}

struct CodeGenerator {
  indent: usize,
  pub code: String
}

impl CodeGenerator {
  pub fn new() -> CodeGenerator {
    CodeGenerator {
      indent: 0,
      code: String::new()
    }
  }

  pub fn open_block(&mut self) {
    self.push_line("{");
    self.indent += 2;
  }

  pub fn close_block(&mut self) {
    self.indent -= 2;
    self.push_line("}");
  }

  pub fn push_line(&mut self, code_line: &str) {
    self.code += &self.indent_spaces();
    self.push_unindented_line(code_line);
  }

  pub fn push_unindented_line(&mut self, code_line: &str) {
    self.code += code_line;
    self.newline();
  }

  pub fn push_java_method(&mut self, code_block: String) {
    let mut lines_iter = code_block.lines();
    self.push_line(lines_iter.next().unwrap());
    for line in lines_iter {
      self.push_unindented_line(line);
    }
    self.newline();
  }

  pub fn push_block(&mut self, code_block: String) {
    for line in code_block.lines() {
      self.push_line(line);
    }
  }

  fn newline(&mut self) {
    self.code += "\n";
  }

  fn indent_spaces(&self) -> String {
    let mut res = String::new();
    for _ in 0..self.indent {
      res.push(' ');
    }
    res
  }
}
