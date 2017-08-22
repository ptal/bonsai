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
use back::code_formatter::*;
use std::collections::HashMap;

/// Useful to compile expression without using the environment (for example when initializing a field).
/// Precondition: All the free variables occuring in `expr` are supposed to be in scope.
pub fn compile_expression(context: &Context, fmt: &mut CodeFormatter, expr: Expr) {
  ExpressionCompiler::new(context, fmt).compile(expr)
}

/// Wrap the expression inside a closure `(env) -> [[expr]]` to be executed later with the environment.
pub fn compile_closure(context: &Context, fmt: &mut CodeFormatter, expr: Expr, return_expr: bool) {
  ExpressionCompiler::new(context, fmt).closure(expr, return_expr)
}

struct ExpressionCompiler<'a> {
  context: &'a Context<'a>,
  fmt: &'a mut CodeFormatter
}

impl<'a> ExpressionCompiler<'a>
{
  fn new(context: &'a Context, fmt: &'a mut CodeFormatter) -> Self {
    ExpressionCompiler {
      context: context,
      fmt: fmt
    }
  }

  fn compile(&mut self, expr: Expr) {
    use ast::ExprKind::*;
    match expr.node {
      NewInstance(ty, args) => self.java_new(ty, args),
      CallChain(chain) => self.method_call_chain(chain),
      Boolean(b) => self.boolean(b),
      Number(n) => self.number(n),
      StringLiteral(lit) => self.literal(lit),
      Var(var) => self.variable(var),
      Bottom => panic!("[BUG] `bot` should only appears on RHS of variable declaration (parsing stage).")
    }
  }

  fn var_uid(&mut self, uid: usize) -> String {
    let var = self.context.var_by_uid(uid);
    let fn_name =
      if var.is_field() { "__uid" }
      else { "__proc_uid" };
    format!("{}(\"{}\")", fn_name, var.name)
  }

  /// A closure is generated each time we call a Java expression or need the value of a variable.
  /// The closure is needed for retrieving these values from the environment.
  fn closure(&mut self, expr: Expr, return_expr: bool) {
    self.fmt.push("(env) -> ");
    let mut variables = HashMap::new();
    self.collect_variable(&mut variables, expr.clone());
    if !variables.is_empty() {
      self.fmt.terminate_line("{");
      self.fmt.indent();
      for (uid, (name, past)) in variables {
        let ty = self.context.var_by_uid(uid).ty;
        self.fmt.push_line(&format!(
          "{} {} = ({}) env.var(\"{}\", {});",
          ty, name, ty, name, past));
      }
      if return_expr {
        self.fmt.push("return ");
      }
      self.compile(expr);
      self.fmt.unindent();
      self.fmt.push(";}");
    }
    else {
      self.compile(expr);
    }
  }

  /// Collect all the variables appearing in `expr` and insert them in `variables`.
  /// This is used to create a closure of this expression.
  fn collect_variable(&self, variables: &mut HashMap<usize, (Ident, usize)>, expr: Expr) {
    use ast::ExprKind::*;
    match expr.node {
      NewInstance(_, args) => {
        for arg in args {
          self.collect_variable(variables, arg);
        }
      }
      CallChain(chain) => {
        for call in chain.calls {
          if !call.target.is_empty() {
            let uid = call.target.first_uid();
            // Host variables can only appear as fields, and thus do not need to be retrieved from the environment.
            if !self.context.var_by_uid(uid).is_host() {
              variables.insert(uid, (call.target.first(), 0));
            }
          }
          for arg in call.args {
            self.collect_variable(variables, arg);
          }
        }
      }
      Var(var) => { variables.insert(var.first_uid(), (var.first(), var.past)); }
      _ => ()
    }
  }

  fn args_list(&mut self, args: Vec<Expr>) {
    let args_len = args.len();
    self.fmt.push(&"(");
    for (i, arg) in args.into_iter().enumerate() {
      self.compile(arg);
      if i != args_len - 1 {
        self.fmt.push(", ");
      }
    }
    self.fmt.push(")");
  }

  fn java_new(&mut self, ty: JType, args: Vec<Expr>) {
    self.fmt.push(&format!("new {}", ty));
    self.args_list(args);
  }

  fn method_call_chain(&mut self, chain: MethodCallChain) {
    let chain_len = chain.calls.len();
    for (i, call) in chain.calls.into_iter().enumerate() {
      if !call.target.is_empty() {
        self.var_path(call.target);
        self.fmt.push(".");
      }
      self.fmt.push(&format!("{}", call.method));
      self.args_list(call.args);
      if i != chain_len - 1 {
        self.fmt.push(".");
      }
    }
  }

  fn boolean(&mut self, b: bool) {
    self.fmt.push(&format!("{}", b));
  }

  fn number(&mut self, n: u64) {
    self.fmt.push(&format!("{}", n));
  }

  fn literal(&mut self, lit: String) {
    self.fmt.push(&format!("\"{}\"", lit));
  }

  fn variable(&mut self, var: Variable) {
    self.var_path(var.path);
  }

  fn var_path(&mut self, path: VarPath) {
    self.fmt.push(&format!("{}", path));
  }
}
