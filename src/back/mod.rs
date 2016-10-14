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
  generate_execute_process(&mut gen);
  generate_items(&mut gen, ast.items);
  gen.close_block();
  Partial::Value(gen.code)
}

fn generate_items(gen: &mut CodeGenerator, items: Vec<Item>) {
  let items = generate_init_proc(gen, items);
  for item in items {
    match item {
      Item::Statement(_) =>
        unreachable!("Should have been moved into the init process."),
      Item::Proc(process) => generate_process(gen, process),
      Item::JavaStaticMethod(_, method) => gen.push_java_method(method)
    }
  }
}

fn generate_execute_process(gen: &mut CodeGenerator) {
  gen.push_line("public void execute()");
  gen.open_block();
  gen.push_block(String::from("\
    Program program = init();\n\
    SpaceMachine machine = SpaceMachine.create(program);\n\
    machine.execute();"));
  gen.close_block();
  gen.newline();
}

fn generate_init_proc(gen: &mut CodeGenerator, items: Vec<Item>) -> Vec<Item> {
  let mut body = vec![];
  let mut remaining = vec![];
  for item in items {
    match item {
      Item::Statement(stmt) => body.push(stmt),
      item => remaining.push(item)
    }
  }
  let init_proc = Process::new(String::from("init"),
    String::from("()"), Stmt::Seq(body));
  generate_process(gen, init_proc);
  remaining
}

fn generate_process(gen: &mut CodeGenerator, process: Process) {
  gen.push_line(&format!(
    "public Program {}{}", process.name, process.params));
  gen.open_block();
  gen.push_line("return");
  gen.indent();
  generate_statement(gen, process.body);
  gen.unindent();
  gen.terminate_line(";");
  gen.close_block();
  gen.newline();
}

fn generate_expr(gen: &mut CodeGenerator, expr: Expr) {
  gen.push("expr");
}

fn generate_statement(gen: &mut CodeGenerator, stmt: Stmt) {
  use ast::Stmt::*;
  match stmt {
    Seq(branches) => generate_sequence(gen, branches),
    Par(branches) => generate_parallel(gen, branches),
    Space(branches) => generate_space(gen, branches),
    Let(let_decl) => generate_let(gen, let_decl),
    LetInStore(let_in_store) => generate_let_in_store(gen, let_in_store),
    When(entailment, body) => generate_when(gen, entailment, body),
    Pause => generate_pause(gen),
    Trap(name, body) => generate_trap(gen, name, body),
    Exit(name) => generate_exit(gen, name),
    Loop(body) => generate_loop(gen, body),
    FnCall(name, args) => generate_fn_call(gen, name, args),
    Tell(var, expr) => generate_tell(gen, var, expr),
  }
}

fn generate_nary_operator(gen: &mut CodeGenerator, op_name: &str, mut branches: Vec<Stmt>) {
  if branches.len() == 1 {
    generate_statement(gen, branches.pop().unwrap());
  }
  else {
    let mid = branches.len() / 2;
    let right = branches.split_off(mid);
    gen.push_line(&format!("SC.{}(", op_name));
    gen.indent();
    generate_sequence(gen, branches);
    gen.terminate_line(",");
    generate_sequence(gen, right);
    gen.push(")");
    gen.unindent();
  }
}

fn generate_sequence(gen: &mut CodeGenerator, branches: Vec<Stmt>) {
  generate_nary_operator(gen, "seq", branches);
}

fn generate_parallel(gen: &mut CodeGenerator, branches: Vec<Stmt>) {
  generate_nary_operator(gen, "merge", branches);
}

fn generate_space(gen: &mut CodeGenerator, branches: Vec<Stmt>) {
  let last_branch = branches.len()-1;
  gen.push_line("new Space(new ArrayList<>(Arrays.asList(");
  for (i, stmt) in branches.into_iter().enumerate() {
    gen.push_line("new SpaceBranch(");
    gen.indent();
    generate_statement(gen, stmt);
    gen.unindent();
    if i != last_branch {
      gen.terminate_line("),");
    }
    else {
      gen.push(")")
    }
  }
  gen.push(")))");
}

fn generate_let(gen: &mut CodeGenerator, let_decl: LetDecl) {
  let spacetime = generate_spacetime(let_decl.spacetime);
  gen.push(&format!("new SpacetimeVar(\"{}\", {}, (env) -> ",
    let_decl.var, spacetime));
  generate_expr(gen, let_decl.expr);
  gen.terminate_line(",");
  generate_statement(gen, *let_decl.body);
  gen.push(")");
}

fn generate_spacetime(spacetime: Spacetime) -> String {
  use ast::Spacetime::*;
  match spacetime {
    SingleSpace => String::from("Spacetime.SingleSpace"),
    SingleTime => String::from("Spacetime.SingleTime"),
    WorldLine => String::from("Spacetime.WorldLine"),
    Location(_) => unreachable!("location is not a translatable spacetime.")
  }
}

fn generate_let_in_store(gen: &mut CodeGenerator, let_in_store: LetInStoreDecl) {
  gen.push(&format!("new LocationVar(\"{}\", \"{}\", (env) -> ",
    let_in_store.location, let_in_store.store));
  generate_expr(gen, let_in_store.expr);
  gen.terminate_line(",");
  generate_statement(gen, *let_in_store.body);
  gen.push(")");
}

fn generate_when(gen: &mut CodeGenerator, entailment: EntailmentRel, body: Box<Stmt>) {
  gen.push("when");
}

fn generate_pause(gen: &mut CodeGenerator) {
  gen.push("pause");
}

fn generate_trap(gen: &mut CodeGenerator, name: String, body: Box<Stmt>) {
  gen.push("trap");
}

fn generate_exit(gen: &mut CodeGenerator, name: String) {
  gen.push("exit");
}

fn generate_loop(gen: &mut CodeGenerator, body: Box<Stmt>) {
  gen.push("loop");
}

fn generate_fn_call(gen: &mut CodeGenerator, name: String, args: Vec<Expr>) {
  gen.push("fn_call");
}

fn generate_tell(gen: &mut CodeGenerator, var: Var, expr: Expr) {
  gen.push("tell");
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

  pub fn push_java_method(&mut self, code_block: String) {
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

  fn newline(&mut self) {
    self.code += "\n";
  }

  fn push_indent(&mut self) {
    for _ in 0..self.indent {
      self.code.push(' ');
    }
  }
}
