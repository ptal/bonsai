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

/// For each variable, depending on its context, we infer its permission if not explicitly written by the user.
/// Permissions can either be `Read`, `Write` or `ReadWrite` if the variable is accessed in read-only, write-only or both.
/// For arguments of external methods, we do not know how the method is actually modifying the variable, it is the responsibility of the user to use them according to the permission.
/// By default, if not precised, the permission is `Read`.
/// In a tell statement `x <- e`, `x` is write only.
/// In an entailment condition `e |= e'`, every variable appearing in `e` or `e'` are supposed to be read-only.

/// In addition, we detect two errors:
///   1. We forbid `pre` on module, host and single_time variables.
///   2. We forbid to write on `pre` variables.

use context::*;
use session::*;
use ast::Permission::*;

pub fn infer_permission(session: Session, context: Context) -> Env<Context> {
  let permission = InferPermission::new(session, context);
  permission.compute()
}

struct InferPermission {
  session: Session,
  context: Context,
  perm_context: Permission,
  context_span: Span
}

impl InferPermission {
  pub fn new(session: Session, context: Context) -> Self {
    InferPermission {
      session: session,
      context: context,
      perm_context: ReadWrite,
      context_span: DUMMY_SP
    }
  }

  fn session<'a>(&'a mut self) -> &'a mut Session {
    &mut self.session
  }

  fn compute(mut self) -> Env<Context> {
    let mut bcrate_clone = self.context.clone_ast();
    self.visit_crate(&mut bcrate_clone);
    self.context.replace_ast(bcrate_clone);
    if self.session.has_errors() {
      Env::fake(self.session, self.context)
    } else {
      Env::value(self.session, self.context)
    }
  }

  // Returns false if an error occurred.
  fn check_pre_on_variable(&mut self, var: &Variable) -> bool {
    if var.past == 0 {
      true
    }
    else {
      let uid = var.last_uid();
      match self.context.var_by_uid(uid).kind {
        Kind::Product => self.err_forbid_pre_on(var, "module"),
        Kind::Host => self.err_forbid_pre_on(var, "host"),
        Kind::Spacetime(Spacetime::SingleTime) => self.err_forbid_pre_on(var, "single_time"),
        _ => {
          // Forbid to write `pre x <- 1`.
          if self.perm_context == Write {
            self.err_forbid_write_on_pre(var);
          }
          else {
            return true;
          }
        }
      }
      false
    }
  }

  fn check_permission(&mut self, var: &Variable, perm: Permission) {
    match (perm, self.perm_context) {
      // Read is allowed in read and readwrite contexts.
      (Read, Write)
      // Readwrite is only allowed in a readwrite context.
    | (ReadWrite, Write) | (ReadWrite, Read)
      // Write is allowed in write and readwrite contexts.
    | (Write, Read) => self.err_illegal_permission_in_context(var, perm),
      _ => ()
    }
  }

  fn check_host_function(&mut self) -> bool {
    if self.perm_context == Read {
      self.err_forbid_host_in_read_context();
      false
    }
    else { true }
  }

  fn visit_method_call_target(&mut self, target: &mut Variable) {
    // If we do not have information on the target (call on host function), we set the permission to READ.
    // For example: `System.out.println("a")` becomes `read System.out.println("a")`.
    if target.last_uid() == 0 {
      if target.permission.is_some() {
        self.err_forbid_permission_on_host_path(target);
      }
      target.permission = Some(Read);
    }
    else {
      self.visit_var(target);
    }
  }

  fn err_forbid_host_in_read_context(&mut self) {
    let sp = self.context_span;
    self.session().struct_span_err_with_code(sp,
      &format!("illegal host function in a read only context."),
      "E0027")
    .help(&"Host function cannot be called inside an entailment expression.\n\
            Solution: call the host function outside of the entailment expression.")
    .emit();
  }

  fn err_forbid_write_on_pre(&mut self, var: &Variable) {
    self.session().struct_span_err_with_code(var.span,
      &format!("forbidden write on `pre` variable."),
      "E0016")
    .span_label(var.span, &format!("write here"))
    .help(&"`pre` variables can only be read.")
    .emit();
  }

  fn err_forbid_pre_on(&mut self, var: &Variable, kind: &str) {
    self.session().struct_span_err_with_code(var.span,
      &format!("illegal kind of the variable `{}`.", var.last()),
      "E0017")
    .span_label(var.last().span, &format!("this variables has the kind `{}`", kind))
    .help(&"Only `single_space` and `world_line` variables can be used under the `pre` operator.")
    .emit();
  }

  fn err_illegal_permission_in_context(&mut self, var: &Variable, perm: Permission) {
    let context_msg = match self.perm_context {
      Write => "The operator `<-` only accept write permission on its left.",
      Read => "The operator `|=` only accept variables with a `read` permission.\n\
               Solution: write on the variables before the entailment test (outside the conditional statement).",
      ReadWrite => unreachable!("errors are not generated with a readwrite context because it is the most general."),
    };
    let perm_context = self.perm_context;
    self.session().struct_span_err_with_code(var.span,
      &format!("illegal permission of the variable `{}`.", var.last()),
      "E0026")
    .span_label(var.last().span, &format!(
      "this variables is accessed with the permission `{}` in a `{}` context.", perm, perm_context))
    .help(&context_msg)
    .emit();
  }

  fn err_forbid_permission_on_host_path(&mut self, var: &Variable) {
    self.session().struct_span_err_with_code(var.span,
      &format!("illegal permission on this path."),
      "E0034")
    .help(&"Permission on host paths are forbidden.\n\
            Solution: Remove the permission.\n\
            Rational: Given a host function call `m.a.f()`, we do not have information on `m.a` (e.g. is it a global variable? a package?).\n\
            Semantics: The permission of the target will be semantically equivalent to `read`.\n\
                       Consequently you can call several time `System.out.println`.")
    .emit();
  }

  fn visit_read_only_expr(&mut self, condition: &mut Expr) {
    let old = self.perm_context;
    self.perm_context = Read;
    self.visit_expr(condition);
    self.perm_context = old;
  }
}

impl VisitorMut<JClass> for InferPermission
{
  fn visit_expr(&mut self, expr: &mut Expr) {
    let old = self.context_span;
    self.context_span = expr.span;
    walk_expr_mut(self, expr);
    self.context_span = old;
  }

  fn visit_var(&mut self, var: &mut Variable) {
    if self.check_pre_on_variable(var) {
      match var.permission.clone() {
        Some(p) => self.check_permission(var, p),
        None =>
          // By default variable has the permission `Read` unless they are in a `Write` only context (e.g. a tell).
          match self.perm_context.clone() {
            Write => var.permission = Some(Write),
            _ => var.permission = Some(Read)
          }
      }
    }
  }

  fn visit_stmt(&mut self, child: &mut Stmt) {
    let old = self.context_span;
    self.context_span = child.span;
    walk_stmt_mut(self, child);
    self.context_span = old;
  }

  fn visit_when(&mut self, condition: &mut Expr, then_branch: &mut Stmt, else_branch: &mut Stmt) {
    self.visit_read_only_expr(condition);
    self.visit_stmt(then_branch);
    self.visit_stmt(else_branch);
  }

  fn visit_suspend(&mut self, suspend: &mut SuspendStmt) {
    self.visit_read_only_expr(&mut suspend.condition);
    self.visit_stmt(&mut *suspend.body)
  }

  fn visit_abort(&mut self, condition: &mut Expr, child: &mut Stmt) {
    self.visit_read_only_expr(condition);
    self.visit_stmt(child)
  }

  fn visit_tell(&mut self, var: &mut Variable, expr: &mut Expr) {
    let old = self.perm_context;
    self.perm_context = Write;
    self.visit_var(var);
    self.perm_context = ReadWrite;
    self.visit_expr(expr);
    self.perm_context = old;
  }

  fn visit_entailment(&mut self, rel: &mut EntailmentRel) {
    let old = self.perm_context;
    self.perm_context = Read;
    self.visit_expr(&mut rel.left);
    self.visit_expr(&mut rel.right);
    self.perm_context = old;
  }

  fn visit_new_instance(&mut self, _ty: JType, args: &mut Vec<Expr>) {
    if self.check_host_function() {
      walk_exprs_mut(self, args);
    }
  }

  fn visit_method_call(&mut self, call: &mut MethodCall) {
    if self.check_host_function() {
      if let Some(ref mut target) = call.target {
        self.visit_method_call_target(target);
      }
      walk_exprs_mut(self, &mut call.args)
    }
  }
}
