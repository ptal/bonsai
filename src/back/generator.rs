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
use visitor::*;
use partial::*;
use driver::config::*;
use back::code_formatter::*;
use back::context::*;
use std::collections::{HashSet, HashMap};

pub fn generate_runtime(module: JModule, stream_bound: HashMap<String, usize>,
  config: &Config) -> Partial<String>
{
  let context = Context::new(module.clone(), stream_bound, config.debug);
  let mut fmt = CodeFormatter::new();
  fmt.push_block(module.host.header.clone());
  fmt.push_line(&format!("package {};", module.host.package));
  for import in &module.host.imports {
    fmt.push_line(&format!("import {};", import));
  }
  fmt.push(&format!("public class {} implements Executable", module.host.class_name));
  fmt.terminate_line(&list_of_interfaces(module.host.interfaces.clone()));
  fmt.open_block();
  for attr in module.host.java_attrs.clone() {
    generate_java_attr(&mut fmt, attr);
  }
  // Extract all local variable (and module variable) as class attributes.
  LocalAttr::generate_local_attr(&mut fmt, module.clone());
  generate_main_method(config, &mut fmt, module.host.class_name);
  for process in module.processes {
    generate_process(&mut fmt, &context, process);
  }
  for method in module.host.java_methods {
    generate_java_method(&mut fmt, method);
  }
  for constructor in module.host.java_constructors {
    generate_java_constructor(&mut fmt, constructor);
  }
  fmt.close_block();
  Partial::Value(fmt.unwrap())
}

fn list_of_interfaces(interfaces: Vec<JType>) -> String {
  let mut s = String::new();
  for interface in interfaces {
    s.push_str(&format!(", {}", interface));
  }
  s
}

fn generate_main_method(config: &Config, fmt: &mut CodeFormatter, class_name: String) {
  if let Some(main_class) = config.main_method.clone() {
    if main_class == class_name {
      fmt.push_line("public static void main(String[] args)");
      fmt.open_block();
      let machine_method = if config.debug { "createDebug" } else { "create" };
      fmt.push_block(format!("\
        {} current = new {}();\n\
        Program program = current.execute();\n\
        SpaceMachine machine = SpaceMachine.{}(program);\n\
        machine.execute();", class_name.clone(), class_name, machine_method));
      fmt.close_block();
      fmt.newline();
    }
  }
}

fn generate_java_method(fmt: &mut CodeFormatter, method: JMethod) {
  let code = vec![
    format!("{} ", method.visibility),
    string_from_static(method.is_static),
    format!("{} ", method.return_ty),
    method.name,
    method.parameters,
    method.body
  ].iter().flat_map(|x| x.chars()).collect();
  fmt.push_java_method(code);
}

fn generate_java_constructor(fmt: &mut CodeFormatter, constructor: JConstructor) {
  let code = vec![
    format!("{} ", constructor.visibility),
    constructor.name,
    constructor.parameters,
    constructor.body
  ].iter().flat_map(|x| x.chars()).collect();
  fmt.push_java_method(code);
}

fn generate_java_attr(fmt: &mut CodeFormatter, attr: JAttribute) {
  let code: String = vec![
    string_from_final(attr.is_final),
    format!("{} ", attr.visibility),
    string_from_static(attr.is_static),
    format!("{} ", attr.ty),
    attr.name
  ].iter().flat_map(|x| x.chars()).collect();
  fmt.push(&code);
  if let Some(expr) = attr.expr {
    fmt.push(" = ");
    generate_expr(fmt, expr);
  }
  fmt.terminate_line(";");
}

struct LocalAttr<'a> {
  fmt: &'a mut CodeFormatter
}

impl<'a> LocalAttr<'a> {
  pub fn generate_local_attr(fmt: &'a mut CodeFormatter,
    module: JModule)
  {
    LocalAttr {
      fmt: fmt
    }
    .visit_module(module)
  }
}

impl<'a> Visitor<JClass, ()> for LocalAttr<'a> {
  unit_visitor_impl!(bcrate, JClass);
  unit_visitor_impl!(module, JClass);
  unit_visitor_impl!(sequence);
  unit_visitor_impl!(parallel);
  unit_visitor_impl!(space);
  unit_visitor_impl!(tell);
  unit_visitor_impl!(pause);
  unit_visitor_impl!(pause_up);
  unit_visitor_impl!(stop);
  unit_visitor_impl!(exit);
  unit_visitor_impl!(proc_call);
  unit_visitor_impl!(fn_call);
  unit_visitor_impl!(module_call);
  unit_visitor_impl!(nothing);

  fn visit_binding(&mut self, binding: LetBindingBase) {
    let jattr = JAttribute {
      visibility: binding.visibility,
      is_static: false,
      is_final: true,
      ty: binding.ty.clone(),
      name: binding.name,
      expr: Some(Expr::new(DUMMY_SP, ExprKind::JavaNew(binding.ty, vec![]))),
      span: binding.span
    };
    generate_java_attr(self.fmt, jattr);
  }
}

fn string_from_final(is_final: bool) -> String {
  if is_final {
    String::from("final ")
  }
  else { String::new() }
}

fn string_from_static(is_static: bool) -> String {
  if is_static {
    String::from("static ")
  }
  else { String::new() }
}

fn generate_process(fmt: &mut CodeFormatter, context: &Context, process: Process) {
  fmt.push_line(&format!(
    "{} Program {}{}", process.visibility, process.name, process.params));
  fmt.open_block();
  fmt.push_line("return");
  fmt.indent();
  generate_statement(fmt, context, process.body);
  fmt.unindent();
  fmt.terminate_line(";");
  fmt.close_block();
  fmt.newline();
}

/// A closure is generated each time we call a Java expression or need the value of a variable.
/// The closure is needed for retrieving these values from the environment.
fn generate_closure(fmt: &mut CodeFormatter, context: &Context, return_expr: bool, expr: Expr) {
  fmt.push("(env) -> ");
  let mut variables = HashSet::new();
  collect_variable(context, &mut variables, expr.clone());
  if !variables.is_empty() {
    fmt.terminate_line("{");
    fmt.indent();
    for var in variables {
      let ty = context.type_of_var(&var);
      fmt.push_line(&format!(
        "{} {} = ({}) env.var(\"{}\", {});",
        ty, var.name(), ty, var.name(), var.past));
    }
    if return_expr {
      fmt.push("return ");
    }
    generate_expr(fmt, expr);
    fmt.unindent();
    fmt.push(";}");
  }
  else {
    generate_expr(fmt, expr);
  }
}

/// Collect all the variables appearing in `expr` and insert them in `variables`.
fn collect_variable(context: &Context, variables: &mut HashSet<StreamVar>, expr: Expr) {
  use ast::ExprKind::*;
  match expr.node {
    JavaNew(_, args) => {
      for arg in args {
        collect_variable(context, variables, arg);
      }
    }
    JavaObjectCall(object, methods) => {
      if context.is_bonsai_var(&object) {
        variables.insert(StreamVar::simple(expr.span, object));
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

fn generate_expr(fmt: &mut CodeFormatter, expr: Expr) {
  use ast::ExprKind::*;
  match expr.node {
    JavaNew(ty, args) => generate_java_new(fmt, ty, args),
    JavaObjectCall(object, methods) => generate_java_object_call(fmt, object, methods),
    JavaThisCall(method) => generate_java_this_call(fmt, method),
    Boolean(b) => generate_boolean(fmt, b),
    Number(n) => generate_number(fmt, n),
    StringLiteral(lit) => generate_literal(fmt, lit),
    Variable(var) => generate_stream_var(fmt, var),
    Bottom(ty) => generate_bottom(fmt, ty)
  }
}

fn generate_fun_call(fmt: &mut CodeFormatter, name: String, args: Vec<Expr>) {
  let args_len = args.len();
  fmt.push(&format!("{}(", name));
  for (i, arg) in args.into_iter().enumerate() {
    generate_expr(fmt, arg);
    if i != args_len - 1 {
      fmt.push(", ");
    }
  }
  fmt.push(")");
}

fn generate_java_new(fmt: &mut CodeFormatter, ty: JType, args: Vec<Expr>) {
  fmt.push("new ");
  generate_fun_call(fmt, format!("{}", ty), args);
}

fn generate_java_object_call(fmt: &mut CodeFormatter, object: String,
  methods: Vec<JavaCall>)
{
  let methods_len = methods.len();
  fmt.push(&format!("{}.", object));
  for (i, method) in methods.into_iter().enumerate() {
    generate_java_this_call(fmt, method);
    if i != methods_len - 1 {
      fmt.push(".");
    }
  }
}

fn generate_java_this_call(fmt: &mut CodeFormatter, method: JavaCall) {
  if method.is_attribute {
    fmt.push(&method.property);
  }
  else {
    generate_fun_call(fmt, method.property, method.args);
  }
}

fn generate_boolean(fmt: &mut CodeFormatter, b: bool) {
  fmt.push(&format!("{}", b));
}

fn generate_number(fmt: &mut CodeFormatter, n: u64) {
  fmt.push(&format!("{}", n));
}

fn generate_literal(fmt: &mut CodeFormatter, lit: String) {
  fmt.push(&format!("\"{}\"", lit));
}

fn generate_stream_var(fmt: &mut CodeFormatter, var: StreamVar) {
  fmt.push(&var.name());
}

fn generate_bottom(fmt: &mut CodeFormatter, ty: JType) {
  fmt.push(&format!("new {}()", ty.name));
}

fn generate_statement(fmt: &mut CodeFormatter, context: &Context, stmt: Stmt) {
  use ast::StmtKind::*;
  match stmt.node {
    Seq(branches) => generate_sequence(fmt, context, branches),
    Par(branches) => generate_parallel(fmt, context, branches),
    Space(branches) => generate_space(fmt, context, branches),
    Let(body) => generate_let(fmt, context, body),
    When(entailment, body) => generate_when(fmt, context, entailment, body),
    Suspend(entailment, body) => generate_suspend(fmt, context, entailment, body),
    Pause => generate_pause(fmt),
    PauseUp => generate_pause_up(fmt),
    Stop => generate_stop(fmt),
    Trap(name, body) => generate_trap(fmt, context, name, body),
    Exit(name) => generate_exit(fmt, name),
    Loop(body) => generate_loop(fmt, context, body),
    FnCall(java_call) => generate_java_call(fmt, context, java_call),
    ProcCall(process, args) => generate_fun_call(fmt, process, args),
    ModuleCall(run_expr) => generate_module_call(fmt, context, run_expr),
    Tell(var, expr) => generate_tell(fmt, context, var, expr),
    Universe(body) => generate_universe(fmt, context, body),
    Nothing => generate_nothing(fmt)
  }
}

fn generate_nary_operator(fmt: &mut CodeFormatter, context: &Context,
  op_name: &str, mut branches: Vec<Stmt>)
{
  if branches.len() == 1 {
    generate_statement(fmt, context, branches.pop().unwrap());
  }
  else {
    let mid = branches.len() / 2;
    let right = branches.split_off(mid);
    fmt.push_line(&format!("SC.{}(", op_name));
    fmt.indent();
    generate_nary_operator(fmt, context, op_name, branches);
    fmt.terminate_line(",");
    generate_nary_operator(fmt, context, op_name, right);
    fmt.push(")");
    fmt.unindent();
  }
}

fn generate_sequence(fmt: &mut CodeFormatter, context: &Context, branches: Vec<Stmt>) {
  generate_nary_operator(fmt, context, "seq", branches);
}

fn generate_parallel(fmt: &mut CodeFormatter, context: &Context, branches: Vec<Stmt>) {
  generate_nary_operator(fmt, context, "merge", branches);
}

fn generate_space(fmt: &mut CodeFormatter, context: &Context, branches: Vec<Stmt>) {
  let branches_len = branches.len();
  fmt.push_line("new Space(new ArrayList<>(Arrays.asList(");
  fmt.indent();
  for (i, stmt) in branches.into_iter().enumerate() {
    fmt.push_line("new SpaceBranch(");
    fmt.indent();
    generate_statement(fmt, context, stmt);
    fmt.unindent();
    if i != branches_len - 1 {
      fmt.terminate_line("),");
    }
    else {
      fmt.push(")")
    }
  }
  fmt.unindent();
  fmt.push(")))");
}

fn generate_let(fmt: &mut CodeFormatter, context: &Context, let_decl: LetStmt) {
  match let_decl.binding {
    LetBinding::InStore(decl) => generate_let_in_store(fmt, context, decl.binding, decl.store),
    LetBinding::Spacetime(decl) => generate_spacetime_binding(fmt, context, decl.binding, decl.spacetime, decl.is_transient),
    LetBinding::Module(decl) => generate_spacetime_binding(fmt, context, decl.binding, Spacetime::SingleSpace, false)
  }
  fmt.terminate_line(",");
  generate_statement(fmt, context, *let_decl.body);
  fmt.push(")");
}

fn generate_spacetime_binding(fmt: &mut CodeFormatter, context: &Context,
  binding: LetBindingBase, spacetime: Spacetime, is_transient: bool)
{
  let spacetime = generate_spacetime(spacetime);
  let stream_bound = context.stream_bound_of(&binding.name);
  fmt.push(&format!("new SpacetimeVar({}, \"{}\", {}, {}, {}, {}, ",
    binding.name, binding.name, binding.is_module_attr, spacetime,
    is_transient, stream_bound));
  generate_closure(fmt, context, true, binding.expr);
}

fn generate_spacetime(spacetime: Spacetime) -> String {
  use ast::Spacetime::*;
  match spacetime {
    SingleSpace => String::from("Spacetime.SingleSpace"),
    SingleTime => String::from("Spacetime.SingleTime"),
    WorldLine => String::from("Spacetime.WorldLine")
  }
}

fn generate_let_in_store(fmt: &mut CodeFormatter, context: &Context,
  binding: LetBindingBase, store: VarPath)
{
  panic!("Not yet implemented.");
  // fmt.push(&format!("new LocationVar({}, \"{}\", \"{}\", ",
  //   binding.name, binding.name, store.name()));
  // generate_closure(fmt, context, true, binding.expr);
}

fn generate_entailment(fmt: &mut CodeFormatter, context: &Context, entailment: EntailmentRel) {
  fmt.push(&format!("new EntailmentConfig({}, \"", entailment.strict));
  generate_stream_var(fmt, entailment.left.clone());
  fmt.push(&format!("\", {}, ", entailment.left.past));
  generate_closure(fmt, context, true, entailment.right);
  fmt.push(")");
}

fn generate_meta_entailment(fmt: &mut CodeFormatter, context: &Context, rel: MetaEntailmentRel) {
  fmt.push("new MetaEntailmentConfig(");
  generate_entailment(fmt, context, rel.left);
  fmt.push(&format!(", {})", rel.right));
}

fn generate_condition(fmt: &mut CodeFormatter, context: &Context, condition: Condition) {
  match condition {
    Condition::Entailment(rel) => generate_entailment(fmt, context, rel),
    Condition::MetaEntailment(rel) => generate_meta_entailment(fmt, context, rel)
  }
}

fn generate_when(fmt: &mut CodeFormatter, context: &Context,
  condition: Condition, body: Box<Stmt>)
{
  fmt.push("SC.when(");
  generate_condition(fmt, context, condition);
  fmt.terminate_line(",");
  fmt.indent();
  generate_statement(fmt, context, *body);
  fmt.terminate_line(",");
  fmt.push("SC.nothing())");
  fmt.unindent();
}

fn generate_suspend(fmt: &mut CodeFormatter, context: &Context,
  condition: Condition, body: Box<Stmt>)
{
  fmt.push("new SuspendWhen(");
  generate_condition(fmt, context, condition);
  fmt.terminate_line(",");
  fmt.indent();
  generate_statement(fmt, context, *body);
  fmt.push(")");
  fmt.unindent();
}


fn generate_module_call(fmt: &mut CodeFormatter, context: &Context, run_expr: RunExpr) {
  fmt.push(&format!("new RunModule("));
  let expr = run_expr.to_expr();
  generate_closure(fmt, context, true, expr);
  fmt.push(")");
}

fn generate_tell(fmt: &mut CodeFormatter, context: &Context, var: StreamVar, expr: Expr) {
  fmt.push("new Tell(\"");
  generate_stream_var(fmt, var);
  fmt.push("\", ");
  generate_closure(fmt, context, true, expr);
  fmt.push(")");
}

fn generate_pause(fmt: &mut CodeFormatter) {
  fmt.push("SC.stop()");
}

fn generate_pause_up(fmt: &mut CodeFormatter) {
  fmt.push("new PauseUp()");
}

fn generate_stop(fmt: &mut CodeFormatter) {
  fmt.push("new BStop()");
}

fn generate_nothing(fmt: &mut CodeFormatter) {
  fmt.push("SC.NOTHING");
}

fn generate_loop(fmt: &mut CodeFormatter, context: &Context, body: Box<Stmt>) {
  fmt.push_line("SC.loop(");
  fmt.indent();
  generate_statement(fmt, context, *body);
  fmt.unindent();
  fmt.push(")");
}

fn generate_java_call(fmt: &mut CodeFormatter, context: &Context, java_call: Expr) {
  fmt.push("new ClosureAtom(");
  generate_closure(fmt, context, false, java_call);
  fmt.push(")");
}

fn generate_trap(fmt: &mut CodeFormatter, context: &Context,
  name: String, body: Box<Stmt>)
{
  fmt.push_line(&format!("SC.until(\"{}\",", name));
  fmt.indent();
  generate_statement(fmt, context, *body);
  fmt.unindent();
  fmt.push(")");
}

fn generate_exit(fmt: &mut CodeFormatter, name: String) {
  fmt.push(&format!("SC.generate(\"{}\")", name));
}

fn generate_universe(fmt: &mut CodeFormatter, context: &Context, body: Box<Stmt>) {
  fmt.push_line(&format!("new Universe({},", context.debug));
  fmt.indent();
  generate_statement(fmt, context, *body);
  fmt.unindent();
  fmt.push(")");
}
