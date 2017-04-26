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

use context::*;
use back::code_formatter::*;
use std::collections::{HashSet};

pub fn generate_module<'a>(context: &Context, module: JModule) -> Partial<String> {
  let mut fmt = CodeFormatter::new();
  fmt.push_block(module.host.header.clone());
  fmt.push_line(&format!("package {};", module.host.package));
  for import in &module.host.imports {
    fmt.push_line(&format!("import {};", import));
  }
  fmt.push(&format!("public class {}", module.host.class_name));
  generate_interfaces(&mut fmt, module.host.interfaces.clone());
  fmt.newline();
  fmt.open_block();
  for field in module.fields.clone() {
    generate_field(&mut fmt, field);
  }
  generate_object_uid(&mut fmt, &module);
  generate_init_method(&mut fmt, context, &module);
  generate_destroy_method(&mut fmt, context, &module);
  generate_main_method(&mut fmt, context, module.host.class_name);
  for process in module.processes {
    generate_process(&mut fmt, context, process);
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

fn generate_interfaces(fmt: &mut CodeFormatter, interfaces: Vec<JType>) {
  if !interfaces.is_empty() {
    fmt.push(" implements ");
    let len = interfaces.len();
    for (i, interface) in interfaces.into_iter().enumerate() {
      fmt.push(&format!("{}", interface));
      if i != len - 1 {
        fmt.push(", ");
      }
    }
  }
}

fn generate_main_method(fmt: &mut CodeFormatter, context: &Context, class_name: Ident) {
  if let Some(main) = context.config().main_method.clone() {
    if main.class == *class_name {
      fmt.push_line("public static void main(String[] args)");
      fmt.open_block();
      let machine_method = if context.config().debug { "createDebug" } else { "create" };
      fmt.push_block(format!("\
        {} current = new {}();\n\
        Program program = current.{}();\n\
        SpaceMachine machine = SpaceMachine.{}(program);\n\
        machine.execute();", class_name.clone(), class_name, main.method, machine_method));
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
    method.name.unwrap(),
    method.parameters,
    method.body
  ].iter().flat_map(|x| x.chars()).collect();
  fmt.push_java_method(code);
}

fn generate_java_constructor(fmt: &mut CodeFormatter, constructor: JConstructor) {
  let code = vec![
    format!("{} ", constructor.visibility),
    constructor.name.unwrap(),
    constructor.parameters,
    constructor.body
  ].iter().flat_map(|x| x.chars()).collect();
  fmt.push_java_method(code);
}

fn generate_field(fmt: &mut CodeFormatter, field: ModuleField) {
  let code: String = vec![
    string_from_final(field.is_final),
    format!("{} ", field.visibility),
    string_from_static(field.is_static),
    format!("{} ", field.binding.ty),
    field.binding.name.unwrap()
  ].iter().flat_map(|x| x.chars()).collect();
  fmt.push(&code);
  if let Some(expr) = field.binding.expr {
    fmt.push(" = ");
    generate_expr(fmt, expr);
  }
  fmt.terminate_line(";");
}

fn generate_init_method(fmt: &mut CodeFormatter, context: &Context, module: &JModule) {
  fmt.push_line("public void __init(SpaceEnvironment senv)");
  fmt.open_block();
  fmt.push_line("__num_instances++;");
  fmt.push_line("__object_instance = __num_instances;");
  for field in module.fields.clone() {
    let binding = field.binding;
    if binding.is_module() {
      fmt.push_line(&format!("{}.__init(senv);", binding.name));
    }
    else {
      fmt.push("senv.enterScope(");
      generate_binding(fmt, context, binding, true, "__uid");
      fmt.terminate_line(");");
    }
  }
  fmt.close_block();
}

fn generate_destroy_method(fmt: &mut CodeFormatter, _context: &Context, module: &JModule) {
  fmt.push_line("public void __destroy(SpaceEnvironment senv)");
  fmt.open_block();
  for field in module.fields.clone() {
    let binding = field.binding;
    if binding.is_module() {
      fmt.push_line(&format!("{}.__destroy(senv);", binding.name));
    }
    else {
      fmt.push_line(&format!("senv.exitScope(__uid(\"{}\"));", binding.name));
    }
  }
  fmt.close_block();
}

fn generate_object_uid(fmt: &mut CodeFormatter, module: &JModule) {
  fmt.push_line("private static int __num_instances = -1;");
  fmt.push_line("private int __object_instance;");
  fmt.push_line("public String __uid(String var)");
  fmt.open_block();
  // return "[package]." + "[classname]." + __object_instance + "." + var;
  fmt.push_line(&format!(
    "return \"{}.\" + \"{}.\" + __object_instance + \".\" + var;",
    module.host.package, module.host.class_name));
  fmt.close_block();
}

fn generate_proc_uid(fmt: &mut CodeFormatter, process: &Process, proc_instance: String) {
  fmt.push_line(&format!("{}++;", proc_instance));
  fmt.push_line(&format!("int __proc_instance = {};", proc_instance));
  fmt.push_line("java.util.function.Function<String, String> __proc_uid = ");
  fmt.push_line(&format!(
    "  (var) -> __uid(\"{}.\" + __proc_instance + \".\" + var);",
    process.name));
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
  let proc_instance = format!("__proc_{}_instance", process.name);
  fmt.push_line(&format!("static int {} = -1;", proc_instance));
  fmt.push_line(&format!(
    "{} Program {}{}", process.visibility, process.name, process.params));
  fmt.open_block();
  generate_proc_uid(fmt, &process, proc_instance);
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

fn generate_fun_call(fmt: &mut CodeFormatter, name: Ident, args: Vec<Expr>) {
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
  generate_fun_call(fmt, Ident::new(ty.span, format!("{}", ty)), args);
}

fn generate_java_object_call(fmt: &mut CodeFormatter, object: Ident,
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
  if method.is_field {
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
  fmt.push(&format!("new LocalVar("));
  generate_binding(fmt, context, let_decl.binding, false, "__proc_uid.apply");
  fmt.terminate_line(",");
  generate_statement(fmt, context, *let_decl.body);
  fmt.push(")");
}

fn generate_binding(fmt: &mut CodeFormatter, context: &Context,
  binding: Binding, is_field: bool, uid_fn: &str)
{
  match binding.kind {
    Kind::Spacetime(spacetime) =>
      generate_spacetime_binding(fmt, context, binding,
        spacetime, is_field, uid_fn),
    Kind::Product =>
      generate_module_binding(fmt, context, binding, uid_fn),
    Kind::Host => panic!(
      "BUG: Host variables are not stored inside the \
       environment, and therefore binding cannot be generated.")
  }
}

fn generate_spacetime_binding(fmt: &mut CodeFormatter, context: &Context,
  binding: Binding, spacetime: Spacetime, is_field: bool, uid_fn: &str)
{
  let spacetime = generate_spacetime(spacetime);
  let stream_bound = context.stream_bound_of(&binding.name);
  fmt.push("new SpacetimeVar(");
  if is_field { fmt.push(&binding.name); }
  else { generate_bottom(fmt, binding.ty.clone()); }
  fmt.push(&format!(",\"{}\", {}(\"{}\"), {}, {}, {},",
    binding.name, uid_fn, binding.name, spacetime,
    binding.is_transient(), stream_bound));
  generate_closure(fmt, context, true,
    binding.expr.expect("BUG: Generate binding without an expression."));
  fmt.push(")");
}

fn generate_module_binding(fmt: &mut CodeFormatter, context: &Context,
  binding: Binding, uid_fn: &str)
{
  fmt.push(&format!("new ModuleVar(\"{}\", {}(\"{}\"), ",
    binding.name, uid_fn, binding.name));
  generate_closure(fmt, context, true,
    binding.expr.expect("BUG: Generate binding without an expression."));
  fmt.push(")");
}

fn generate_spacetime(spacetime: Spacetime) -> String {
  use ast::Spacetime::*;
  match spacetime {
    SingleSpace(_) => String::from("Spacetime.SingleSpace"),
    SingleTime => String::from("Spacetime.SingleTime"),
    WorldLine(_) => String::from("Spacetime.WorldLine")
  }
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
  fmt.push(&format!("new CallProcess("));
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
  name: Ident, body: Box<Stmt>)
{
  fmt.push_line(&format!("SC.until(\"{}\",", name));
  fmt.indent();
  generate_statement(fmt, context, *body);
  fmt.unindent();
  fmt.push(")");
}

fn generate_exit(fmt: &mut CodeFormatter, name: Ident) {
  fmt.push(&format!("SC.generate(\"{}\")", name));
}

fn generate_universe(fmt: &mut CodeFormatter, context: &Context, body: Box<Stmt>) {
  fmt.push_line(&format!("new Universe({},", context.config().debug));
  fmt.indent();
  generate_statement(fmt, context, *body);
  fmt.unindent();
  fmt.push(")");
}
