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

fn generate_closure(gen: &mut CodeGenerator, expr: Expr) {
  gen.push("(env) -> ");
  generate_expr(gen, expr);
}

fn generate_expr(gen: &mut CodeGenerator, expr: Expr) {
  use ast::Expr::*;
  match expr {
    JavaNew(ty, args) => generate_java_new(gen, ty, args),
    JavaObjectCall(object, methods) => generate_java_object_call(gen, object, methods),
    JavaThisCall(method) => generate_java_this_call(gen, method),
    Number(n) => generate_number(gen, n),
    StringLiteral(lit) => generate_literal(gen, lit),
    Variable(var) => generate_stream_var(gen, var)
  }
}

fn generate_fun_call(gen: &mut CodeGenerator, name: String, args: Vec<Expr>) {
  let args_len = args.len();
  gen.push(&format!("{}(", name));
  for (i, arg) in args.into_iter().enumerate() {
    generate_expr(gen, arg);
    if i != args_len - 1 {
      gen.push(", ");
    }
  }
  gen.push(")");
}

fn generate_java_new(gen: &mut CodeGenerator, ty: JavaTy, args: Vec<Expr>) {
  gen.push("new ");
  generate_fun_call(gen, format!("{}", ty), args);
}

fn generate_java_object_call(gen: &mut CodeGenerator, object: String,
  methods: Vec<JavaCall>)
{
  let methods_len = methods.len();
  gen.push(&format!("{}.", object));
  for (i, method) in methods.into_iter().enumerate() {
    generate_java_this_call(gen, method);
    if i != methods_len - 1 {
      gen.push(".");
    }
  }
}

fn generate_java_this_call(gen: &mut CodeGenerator, method: JavaCall) {
  if method.is_attribute {
    gen.push(&method.property);
  }
  else {
    generate_fun_call(gen, method.property, method.args);
  }
}

fn generate_number(gen: &mut CodeGenerator, n: u64) {
  gen.push(&format!("{}", n));
}

fn generate_literal(gen: &mut CodeGenerator, lit: String) {
  gen.push(&format!("\"{}\"", lit));
}

fn generate_stream_var(gen: &mut CodeGenerator, var: StreamVar) {
  gen.push(&var.name);
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
    FnCall(java_call) => generate_java_call(gen, java_call),
    ProcCall(process) => generate_proc_call(gen, process),
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
  let branches_len = branches.len();
  gen.push_line("new Space(new ArrayList<>(Arrays.asList(");
  gen.indent();
  for (i, stmt) in branches.into_iter().enumerate() {
    gen.push_line("new SpaceBranch(");
    gen.indent();
    generate_statement(gen, stmt);
    gen.unindent();
    if i != branches_len - 1 {
      gen.terminate_line("),");
    }
    else {
      gen.push(")")
    }
  }
  gen.unindent();
  gen.push(")))");
}

fn generate_let(gen: &mut CodeGenerator, let_decl: LetDecl) {
  let spacetime = generate_spacetime(let_decl.spacetime);
  gen.push(&format!("new SpacetimeVar(\"{}\", {}, ",
    let_decl.var, spacetime));
  generate_closure(gen, let_decl.expr);
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
  gen.push(&format!("new LocationVar(\"{}\", \"{}\", ",
    let_in_store.location, let_in_store.store));
  generate_closure(gen, let_in_store.expr);
  gen.terminate_line(",");
  generate_statement(gen, *let_in_store.body);
  gen.push(")");
}

fn generate_entailment(gen: &mut CodeGenerator, entailment: EntailmentRel) {
  gen.push(&format!("new EntailmentConfig(\"{}\", ",
    entailment.left.name));
  generate_closure(gen, entailment.right);
  gen.push(")");
}

fn generate_when(gen: &mut CodeGenerator, entailment: EntailmentRel, body: Box<Stmt>) {
  gen.push("SC.when(");
  generate_entailment(gen, entailment);
  gen.terminate_line(",");
  gen.indent();
  generate_statement(gen, *body);
  gen.terminate_line(",");
  gen.push("SC.nothing())");
  gen.unindent();
}

fn generate_tell(gen: &mut CodeGenerator, var: Var, expr: Expr) {
  gen.push(&format!("new Tell(\"{}\", ", var.name));
  generate_closure(gen, expr);
  gen.push(")");
}

fn generate_pause(gen: &mut CodeGenerator) {
  gen.push("SC.stop()");
}

fn generate_loop(gen: &mut CodeGenerator, body: Box<Stmt>) {
  gen.push_line("SC.loop(");
  gen.indent();
  generate_statement(gen, *body);
  gen.unindent();
  gen.push(")");
}

fn generate_java_call(gen: &mut CodeGenerator, java_call: Expr) {
  gen.push("new ClosureAtom(");
  generate_closure(gen, java_call);
  gen.push(")");
}

fn generate_proc_call(gen: &mut CodeGenerator, process: String) {
  gen.push(&format!("{}()", process));
}

fn generate_trap(gen: &mut CodeGenerator, name: String, body: Box<Stmt>) {
  gen.push_line(&format!("SC.until(\"{}\",", name));
  gen.indent();
  generate_statement(gen, *body);
  gen.unindent();
  gen.push(")");
}

fn generate_exit(gen: &mut CodeGenerator, name: String) {
  gen.push(&format!("SC.generate(\"{}\")", name));
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
