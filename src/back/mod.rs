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

use jast::*;
use partial::*;
use driver::config::*;
use std::collections::{HashMap, HashSet};

pub struct SpacetimeVar {
  pub name: String,
  pub ty: JType
}

impl SpacetimeVar {
  pub fn new(name: String, ty: JType) -> Self {
    SpacetimeVar {
      name: name,
      ty: ty
    }
  }
}

pub struct Context {
  spacetime_vars: HashMap<String, SpacetimeVar>
}

impl Context {
  pub fn new(module: JModule) -> Self {
    let mut context = Context {
      spacetime_vars: HashMap::new()
    };
    context.initialize_program(module);
    context
  }

  fn initialize_program(&mut self, module: JModule) {
    for process in module.processes {
      self.initialize_stmt(process.body);
    }
  }

  fn insert_var(&mut self, var: String, ty: JType) {
    let spacetime_var = SpacetimeVar::new(var.clone(), ty);
    self.spacetime_vars.insert(
      var,
      spacetime_var);
  }

  fn initialize_stmts(&mut self, stmts: Vec<Stmt>) {
    for stmt in stmts {
      self.initialize_stmt(stmt);
    }
  }

  fn initialize_stmt(&mut self, stmt: Stmt) {
    use ast::Stmt::*;
    match stmt {
      Let(decl) => {
        self.insert_var(decl.var.name, decl.var.ty);
        self.initialize_stmt(*decl.body);
      }
      LetInStore(decl) => {
        self.insert_var(decl.var.name, decl.var.ty);
        self.initialize_stmt(*decl.body);
      }
      Seq(branches)
    | Par(branches)
    | Space(branches) => self.initialize_stmts(branches),
      When(_, stmt)
    | Trap(_, stmt)
    | Loop(stmt) => self.initialize_stmt(*stmt),
      _ => ()
    }
  }

  pub fn type_of_var(&self, var: &StreamVar) -> JType {
    self.spacetime_vars.get(&var.name)
      .expect(&format!("Undeclared variable `{}`.", var.name))
      .ty.clone()
  }

  pub fn is_spacetime_var(&self, name: &String) -> bool {
    self.spacetime_vars.contains_key(name)
  }
}

pub fn generate_chococubes(module: JModule, config: &Config) -> Partial<String> {
  let context = Context::new(module.clone());
  let mut gen = CodeGenerator::new();
  gen.push_block(module.host.header);
  gen.push_line(&format!("public class {} implements Executable", module.host.class_name));
  gen.open_block();
  for attr in module.host.java_attrs {
    generate_java_attr(&mut gen, attr);
  }
  if config.main_method.is_some() {
    generate_main_method(&mut gen, module.host.class_name, config.debug);
  }
  for process in module.processes {
    generate_process(&mut gen, &context, process);
  }
  for method in module.host.java_methods {
    generate_java_method(&mut gen, method);
  }
  for constructor in module.host.java_constructors {
    generate_java_constructor(&mut gen, constructor);
  }
  gen.close_block();
  Partial::Value(gen.code)
}

fn generate_main_method(gen: &mut CodeGenerator, class_name: String, debug: bool) {
  gen.push_line("public static void main(String[] args)");
  gen.open_block();
  let machine_method = if debug { "createDebug" } else { "create" };
  gen.push_block(format!("\
    {} current = new {}();\n\
    Program program = current.execute();\n\
    SpaceMachine machine = SpaceMachine.{}(program);\n\
    machine.execute();", class_name.clone(), class_name, machine_method));
  gen.close_block();
  gen.newline();
}

fn generate_java_method(gen: &mut CodeGenerator, method: JMethod) {
  let code = vec![
    format!("{} ", method.visibility),
    string_from_static(method.is_static),
    format!("{} ", method.return_ty),
    method.name,
    method.parameters,
    method.body
  ].iter().flat_map(|x| x.chars()).collect();
  gen.push_java_method(code);
}

fn generate_java_constructor(gen: &mut CodeGenerator, constructor: JConstructor) {
  let code = vec![
    format!("{} ", constructor.visibility),
    constructor.name,
    constructor.parameters,
    constructor.body
  ].iter().flat_map(|x| x.chars()).collect();
  gen.push_java_method(code);
}

fn generate_java_attr(gen: &mut CodeGenerator, attr: JAttribute) {
  let code: String = vec![
    format!("{} ", attr.visibility),
    string_from_static(attr.is_static),
    format!("{} ", attr.ty),
    attr.name
  ].iter().flat_map(|x| x.chars()).collect();
  gen.push(&code);
  if let Some(expr) = attr.expr {
    gen.push(" = ");
    generate_expr(gen, expr);
  }
  gen.terminate_line(";");
}

fn string_from_static(is_static: bool) -> String {
  if is_static {
    String::from("static ")
  }
  else { String::new() }
}

fn generate_process(gen: &mut CodeGenerator, context: &Context, process: Process) {
  gen.push_line(&format!(
    "public Program {}{}", process.name, process.params));
  gen.open_block();
  gen.push_line("return");
  gen.indent();
  generate_statement(gen, context, process.body);
  gen.unindent();
  gen.terminate_line(";");
  gen.close_block();
  gen.newline();
}

fn generate_closure(gen: &mut CodeGenerator, context: &Context, return_expr: bool, expr: Expr) {
  gen.push("(env) -> ");
  let mut variables = HashSet::new();
  collect_variable(context, &mut variables, expr.clone());
  if !variables.is_empty() {
    gen.terminate_line("{");
    gen.indent();
    for var in variables {
      let ty = context.type_of_var(&var);
      gen.push_line(&format!(
        "{} {} = ({}) env.var(\"{}\");",
        ty, var.name, ty, var.name));
    }
    if return_expr {
      gen.push("return ");
    }
    generate_expr(gen, expr);
    gen.unindent();
    gen.push(";}");
  }
  else {
    generate_expr(gen, expr);
  }
}

fn collect_variable(context: &Context, variables: &mut HashSet<StreamVar>, expr: Expr) {
  use ast::Expr::*;
  match expr {
    JavaNew(_, args) => {
      for arg in args {
        collect_variable(context, variables, arg);
      }
    }
    JavaObjectCall(object, methods) => {
      if context.is_spacetime_var(&object) {
        variables.insert(StreamVar::simple(object));
      }
      for method in methods {
        for arg in method.args {
          collect_variable(context, variables, arg);
        }
      }
    }
    JavaThisCall(method) => {
      for arg in method.args {
        collect_variable(context, variables, arg);
      }
    }
    Variable(var) => { variables.insert(var); }
    _ => ()
  }
}

fn generate_expr(gen: &mut CodeGenerator, expr: Expr) {
  use ast::Expr::*;
  match expr {
    JavaNew(ty, args) => generate_java_new(gen, ty, args),
    JavaObjectCall(object, methods) => generate_java_object_call(gen, object, methods),
    JavaThisCall(method) => generate_java_this_call(gen, method),
    Number(n) => generate_number(gen, n),
    StringLiteral(lit) => generate_literal(gen, lit),
    Variable(var) => generate_stream_var(gen, var),
    Bottom(ty) => generate_bottom(gen, ty)
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

fn generate_java_new(gen: &mut CodeGenerator, ty: JType, args: Vec<Expr>) {
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

fn generate_bottom(gen: &mut CodeGenerator, ty: JType) {
  gen.push(&format!("new {}()", ty.name));
}

fn generate_statement(gen: &mut CodeGenerator, context: &Context, stmt: Stmt) {
  use ast::Stmt::*;
  match stmt {
    Seq(branches) => generate_sequence(gen, context, branches),
    Par(branches) => generate_parallel(gen, context, branches),
    Space(branches) => generate_space(gen, context, branches),
    Let(let_decl) => generate_let(gen, context, let_decl),
    LetInStore(let_in_store) => generate_let_in_store(gen, context, let_in_store),
    When(entailment, body) => generate_when(gen, context, entailment, body),
    Pause => generate_pause(gen),
    Trap(name, body) => generate_trap(gen, context, name, body),
    Exit(name) => generate_exit(gen, name),
    Loop(body) => generate_loop(gen, context, body),
    FnCall(java_call) => generate_java_call(gen, context, java_call),
    ProcCall(process, args) => generate_fun_call(gen, process, args),
    Tell(var, expr) => generate_tell(gen, context, var, expr),
    Nothing => generate_nothing(gen)
  }
}

fn generate_nary_operator(gen: &mut CodeGenerator, context: &Context,
  op_name: &str, mut branches: Vec<Stmt>)
{
  if branches.len() == 1 {
    generate_statement(gen, context, branches.pop().unwrap());
  }
  else {
    let mid = branches.len() / 2;
    let right = branches.split_off(mid);
    gen.push_line(&format!("SC.{}(", op_name));
    gen.indent();
    generate_nary_operator(gen, context, op_name, branches);
    gen.terminate_line(",");
    generate_nary_operator(gen, context, op_name, right);
    gen.push(")");
    gen.unindent();
  }
}

fn generate_sequence(gen: &mut CodeGenerator, context: &Context, branches: Vec<Stmt>) {
  generate_nary_operator(gen, context, "seq", branches);
}

fn generate_parallel(gen: &mut CodeGenerator, context: &Context, branches: Vec<Stmt>) {
  generate_nary_operator(gen, context, "merge", branches);
}

fn generate_space(gen: &mut CodeGenerator, context: &Context, branches: Vec<Stmt>) {
  let branches_len = branches.len();
  gen.push_line("new Space(new ArrayList<>(Arrays.asList(");
  gen.indent();
  for (i, stmt) in branches.into_iter().enumerate() {
    gen.push_line("new SpaceBranch(");
    gen.indent();
    generate_statement(gen, context, stmt);
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

fn generate_let(gen: &mut CodeGenerator, context: &Context, let_decl: LetStmt) {
  let spacetime = generate_spacetime(let_decl.var.spacetime);
  gen.push(&format!("new SpacetimeVar(\"{}\", {}, ",
    let_decl.var.name, spacetime));
  generate_closure(gen, context, true, let_decl.var.expr);
  gen.terminate_line(",");
  generate_statement(gen, context, *let_decl.body);
  gen.push(")");
}

fn generate_spacetime(spacetime: Spacetime) -> String {
  use ast::Spacetime::*;
  match spacetime {
    SingleSpace => String::from("Spacetime.SingleSpace"),
    SingleTime => String::from("Spacetime.SingleTime"),
    WorldLine => String::from("Spacetime.WorldLine")
  }
}

fn generate_let_in_store(gen: &mut CodeGenerator, context: &Context, let_in_store: LetInStoreStmt) {
  gen.push(&format!("new LocationVar(\"{}\", \"{}\", ",
    let_in_store.var.name, let_in_store.store));
  generate_closure(gen, context, true, let_in_store.var.expr);
  gen.terminate_line(",");
  generate_statement(gen, context, *let_in_store.body);
  gen.push(")");
}

fn generate_entailment(gen: &mut CodeGenerator, context: &Context, entailment: EntailmentRel) {
  gen.push(&format!("new EntailmentConfig(\"{}\", ",
    entailment.left.name));
  generate_closure(gen, context, true, entailment.right);
  gen.push(")");
}

fn generate_when(gen: &mut CodeGenerator, context: &Context,
  entailment: EntailmentRel, body: Box<Stmt>)
{
  gen.push("SC.when(");
  generate_entailment(gen, context, entailment);
  gen.terminate_line(",");
  gen.indent();
  generate_statement(gen, context, *body);
  gen.terminate_line(",");
  gen.push("SC.nothing())");
  gen.unindent();
}

fn generate_tell(gen: &mut CodeGenerator, context: &Context, var: Var, expr: Expr) {
  gen.push(&format!("new Tell(\"{}\", ", var.name));
  generate_closure(gen, context, true, expr);
  gen.push(")");
}

fn generate_pause(gen: &mut CodeGenerator) {
  gen.push("SC.stop()");
}

fn generate_nothing(gen: &mut CodeGenerator) {
  gen.push("SC.NOTHING");
}

fn generate_loop(gen: &mut CodeGenerator, context: &Context, body: Box<Stmt>) {
  gen.push_line("SC.loop(");
  gen.indent();
  generate_statement(gen, context, *body);
  gen.unindent();
  gen.push(")");
}

fn generate_java_call(gen: &mut CodeGenerator, context: &Context, java_call: Expr) {
  gen.push("new ClosureAtom(");
  generate_closure(gen, context, false, java_call);
  gen.push(")");
}

fn generate_trap(gen: &mut CodeGenerator, context: &Context,
  name: String, body: Box<Stmt>)
{
  gen.push_line(&format!("SC.until(\"{}\",", name));
  gen.indent();
  generate_statement(gen, context, *body);
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
