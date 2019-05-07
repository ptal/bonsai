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

use context::*;
use session::*;
use back::code_formatter::*;

/// Useful to compile expression without using the environment (for example when initializing a field).
/// Precondition: All the free variables occuring in `expr` are supposed to be in scope.
pub fn compile_expression(session: &Session, context: &Context, fmt: &mut CodeFormatter, expr: Expr) {
  ExpressionCompiler::new(session, context, fmt).compile(expr, &vec![], None)
}

/// Compile an expression that returns a result.
pub fn compile_functional_expr(session: &Session, context: &Context, fmt: &mut CodeFormatter, expr: Expr, ty: Option<JType>) {
  ExpressionCompiler::new(session, context, fmt).compile_fun_expr(expr, ty)
}

/// Wrap the expression inside a closure `(args) -> [[expr]]` to be executed later with the environment.
pub fn compile_closure(session: &Session, context: &Context, fmt: &mut CodeFormatter, expr: Expr, return_expr: bool) {
  ExpressionCompiler::new(session, context, fmt).closure(expr, return_expr, None)
}

pub fn compile_var_uid(session: &Session, context: &Context, fmt: &mut CodeFormatter, var: Variable) {
  ExpressionCompiler::new(session, context, fmt).var_uid(var)
}

static CLOSURE_ARGS: &str = "__args";
static LOCAL_UID_FN: &str = "__proc_uid.apply";
pub static FIELD_UID_PREFIX: &str = "__uid_";

struct ExpressionCompiler<'a> {
  _session: &'a Session,
  context: &'a Context,
  fmt: &'a mut CodeFormatter
}

impl<'a> ExpressionCompiler<'a>
{
  fn new(_session: &'a Session, context: &'a Context, fmt: &'a mut CodeFormatter) -> Self {
    ExpressionCompiler { _session, context, fmt }
  }

  fn compile_fun_expr (&mut self, expr: Expr, ty: Option<JType>) {
    match expr.node.clone() {
      ExprKind::Entailment(rel) => self.entailment_expr(*rel),
      _ => {
        self.fmt.push("new FunctionCall(");
        self.closure(expr, true, ty);
        self.fmt.push(")");
      }
    }
  }

  fn compile(&mut self, expr: Expr, vars: &Vec<Variable>, ty: Option<JType>) {
    use ast::ExprKind::*;
    match expr.node {
      Var(var) => self.variable(var, vars),
      Call(call) => self.method_call(call, vars),
      NewInstance(new_instance) => self.new_instance(new_instance, vars),
      Trilean(t) => self.trilean(t),
      Number(n) => self.number(n),
      StringLiteral(lit) => self.string_literal(lit),
      Bottom => self.bottom(ty),
      Top => self.top(ty),
      Entailment(rel) => self.entailment(*rel, vars),
      Or(_, _) =>  unimplemented!("trilean or is unimplemented"),
      And(_, _) => unimplemented!("trilean and is unimplemented"),
      Not(_) => unimplemented!("trilean not is unimplemented"),
    }
  }

  fn variable(&mut self, var: Variable, vars: &Vec<Variable>) {
    let v = vars.iter().enumerate().find(|&(_,v)| v.last_uid() == var.last_uid());
    match v {
      // Host variable not registered in the environment.
      // It can also be a static path such as `System.out`.
      None => self.raw_variable(var),
      // Variable passed as arguments at position `__args.get(pos)`.
      Some((pos, _)) => {
        let ty = self.context.var_by_uid(var.last_uid()).ty;
        self.fmt.push(&format!("(({}) ({}.get({})))", ty, CLOSURE_ARGS, pos));
      }
    }
  }

  fn raw_variable(&mut self, var: Variable) {
    if var.with_this {
      self.fmt.push("this.");
    }
    self.fmt.push(&format!("{}", var.path));
  }

  /// A closure is generated each time we call a Java expression or need the value of a variable.
  /// The closure is needed for retrieving these values from the environment.
  fn closure(&mut self, expr: Expr, return_expr: bool, ty: Option<JType>) {
    let mut variables = vec![];
    self.collect_variables(&mut variables, expr.clone());
    self.list_of_accesses(&variables, false);
    self.fmt.terminate_line(&format!(", ({}) -> {{", CLOSURE_ARGS));
    self.fmt.indent();
    if return_expr {
      self.fmt.push("return ");
    }
    self.compile(expr, &variables, ty);
    self.fmt.unindent();
    self.fmt.terminate_line(";}");
  }

  fn list_of_accesses(&mut self, vars: &Vec<Variable>, free_access: bool) {
    self.fmt.terminate_line("Arrays.asList(");
    self.fmt.indent();
    for (i, var) in vars.iter().enumerate() {
      self.var_access(var.clone(), free_access);
      if i != vars.len() - 1 {
        self.fmt.push(",");
      }
    }
    self.fmt.push(")");
    self.fmt.unindent();
  }

  fn var_access(&mut self, var: Variable, free_access: bool) {
    let access_class =
      if free_access || var.permission.is_none() { format!("FreeAccess") }
      else {
        match var.permission.expect(&format!("All variables must have a permission at generation stage ({}).", var)) {
          Permission::Read => format!("ReadAccess"),
          Permission::Write => format!("WriteAccess"),
          Permission::ReadWrite => format!("ReadWriteAccess")
        }
      };
    self.fmt.push(&format!("new {}(", access_class));
    self.var_uid(var);
    self.fmt.push(")");
  }

  fn var_uid(&mut self, var: Variable) {
    let var_info = self.context.var_by_uid(var.first_uid());
    // Variable local to a process.
    if !var_info.is_field() && var.len() == 1 {
      self.local_var(var.first());
    }
    // Variable local to a module.
    else {
      if var.with_this {
        self.fmt.push("this.");
      }
      // Path of the form `m.m2.v`, we first generate the `m.m2` part.
      if var.path.len() > 1 {
        let mut prefix = var.path.clone();
        prefix.fragments.pop();
        self.fmt.push(&format!("{}.", prefix));
      }
      // The UID of the field.
      self.fmt.push(&format!("{}{}",FIELD_UID_PREFIX, var.last()));
    }
  }

  fn local_var(&mut self, var_name: Ident) {
    self.fmt.push(&format!("{}(\"{}\")", LOCAL_UID_FN, var_name));
  }

  /// Collect all the variables appearing in `expr` and insert them in `variables`.
  /// This is used to create a closure of this expression.
  /// `variables[i] = v` represents the variable `v` at position `i` in the expression `expr`.
  fn collect_variables(&self, variables: &mut Vec<Variable>, expr: Expr) {
    use ast::ExprKind::*;
    match expr.node {
      NewInstance(new_instance) => {
        for arg in new_instance.args {
          self.collect_variables(variables, arg);
        }
      }
      Call(call) => {
        if let Some(target) = call.target {
          let uid = target.last_uid();
          // Host variables can only appear as fields, and thus do not need to be retrieved from the environment.
          if self.context.var_by_uid(uid).is_spacetime() {
            variables.push(target);
          }
        }
        for arg in call.args {
          self.collect_variables(variables, arg);
        }
      }
      Var(var) => {
        let var_info = self.context.var_by_uid(var.last_uid());
        if var_info.is_spacetime() {
          variables.push(var);
        }
      }
      And(e1, e2)
    | Or(e1, e2) => {
        self.collect_variables(variables, *e1);
        self.collect_variables(variables, *e2);
      }
      Not(e) => { self.collect_variables(variables, *e); }
      Entailment(entailment) => {
        self.collect_variables(variables,entailment.left.clone());
        self.collect_variables(variables,entailment.right.clone());
      }
      Trilean(_)
    | Bottom | Top
    | Number(_) | StringLiteral(_) => ()
    }
  }

  fn args_list(&mut self, args: Vec<Expr>, vars: &Vec<Variable>) {
    let args_len = args.len();
    self.fmt.push("(");
    for (i, arg) in args.into_iter().enumerate() {
      self.compile(arg, vars, None);
      if i != args_len - 1 {
        self.fmt.push(", ");
      }
    }
    self.fmt.push(")");
  }

  fn method_call(&mut self, call: MethodCall, vars: &Vec<Variable>) {
    if let Some(target) = call.target {
      self.variable(target, vars);
      self.fmt.push(".");
    }
    self.fmt.push(&format!("{}", call.method));
    self.args_list(call.args, vars);
  }

  fn new_instance(&mut self, instance: NewObjectInstance, vars: &Vec<Variable>) {
    self.fmt.push(&format!("new {}", instance.ty));
    self.args_list(instance.args, vars);
  }

  fn trilean(&mut self, t: SKleene) {
    let k = match t {
      SKleene::True => "Kleene.TRUE",
      SKleene::False => "Kleene.FALSE",
      SKleene::Unknown => "Kleene.UNKNOWN"
    };
    self.fmt.push(&format!("new ES({})", k));
  }

  fn number(&mut self, n: u64) {
    self.fmt.push(&format!("{}", n));
  }

  fn string_literal(&mut self, lit: String) {
    self.fmt.push(&format!("\"{}\"", lit));
  }

  fn entailment(&mut self, rel: EntailmentRel, vars: &Vec<Variable>) {
    match rel.op {
      EntailmentKind::StrictEntailment => { panic!("|< is not yet implemented."); }
      // EntailmentKind::Equality => { panic!("== is not yet implemented."); }
      _ => ()
    };
    self.fmt.push("Cast.toLattice(\"<expr in entailment relation>\",");
    self.compile(rel.left, vars, None);
    self.fmt.push(").entails(");
    self.compile(rel.right, vars, None);
    self.fmt.push(")");
  }

  fn entailment_expr(&mut self, rel: EntailmentRel) {
    self.fmt.push("new Entailment(");
    let mut vars_left = vec![];
    let mut vars_right = vec![];
    self.collect_variables(&mut vars_left, rel.left.clone());
    self.collect_variables(&mut vars_right, rel.right.clone());
    self.list_of_accesses(&vars_left, true);
    self.fmt.push(", ");
    self.list_of_accesses(&vars_right, true);
    let mut vars = vars_left;
    vars.extend(vars_right.into_iter());
    self.fmt.push(&format!(", ({}) -> ", CLOSURE_ARGS));
    self.entailment(rel, &vars);
    self.fmt.push(")");
  }

  fn bottom(&mut self, ty: Option<JType>) {
    let ty = self.expect_ty("bot", ty);
    self.fmt.push(&format!("new {}().bottom()", ty));
  }

  fn top(&mut self, ty: Option<JType>) {
    let ty = self.expect_ty("top", ty);
    self.fmt.push(&format!("new {}().top()", ty));
  }

  fn expect_ty(&self, context: &str, ty: Option<JType>) -> JType {
    ty.expect(&format!("[BUG] try to generate the code of `{}` in\
      a context not explicitly typed (e.g. `f({})`).", context, context))
  }
}
