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
/// When calling external method, we do not know how the method is actually modifying the variable so we infer it to `ReadWrite` (which is the safest choice).
/// In a tell statement `x <- e`, `x` is write only.
/// In an entailment condition `e |= e'`, every variable appearing in `e` or `e'` are supposed to be only read.

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

  fn session<'a>(&'a self) -> &'a Session {
    &self.session
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
  fn pre_on_variable(&self, var: &Variable) -> bool {
    let mut has_err = true;
    if var.past > 0 {
      let uid = var.last_uid();
      match self.context.var_by_uid(uid).kind {
        Kind::Product => { self.err_forbid_pre_on(var, "module"); has_err = false }
        Kind::Host => { self.err_forbid_pre_on(var, "host"); has_err = false }
        Kind::Spacetime(Spacetime::SingleTime) => { self.err_forbid_pre_on(var, "single_time"); has_err = false }
        _ => {
          // Forbid to write `pre x <- 1`.
          if self.perm_context == Write {
            self.err_forbid_write_on_pre(var);
            has_err = false;
          }
        }
      }
    }
    has_err
  }

  fn check_permission(&self, var: &Variable, perm: Permission) {
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

  fn err_forbid_write_on_pre(&self, var: &Variable) {
    self.session().struct_span_err_with_code(var.span,
      &format!("forbidden write on `pre` variable."),
      "E0016")
    .span_label(var.span, &format!("write here"))
    .help(&"`pre` variables can only be read.")
    .emit();
  }

  fn err_forbid_pre_on(&self, var: &Variable, kind: &str) {
    self.session().struct_span_err_with_code(var.span,
      &format!("illegal kind of the variable `{}`.", var.last()),
      "E0017")
    .span_label(var.last().span, &format!("this variables has the kind `{}`", kind))
    .help(&"Only `single_space` and `world_line` variables can be used under the `pre` operator.")
    .emit();
  }

  fn err_illegal_permission_in_context(&self, var: &Variable, perm: Permission) {
    let context_msg = match self.perm_context {
      Write => "The operator `<-` only accept write permission on its left.",
      Read => "The operator `|=` only accept variables with a `read` permission.\n\
               Solution: write on the variables before the entailment test (outside the conditional statement).",
      ReadWrite => unreachable!("errors are not generated with a readwrite context because it is the most general."),
    };

    self.session().struct_span_err_with_code(var.span,
      &format!("illegal permission of the variable `{}`.", var.last()),
      "E0026")
    .span_label(var.last().span, &format!(
      "this variables is accessed with the permission `{}` in a `{}` context.", perm, self.perm_context))
    .help(&context_msg)
    .emit();
  }
}

impl VisitorMut<JClass> for InferPermission
{
  fn visit_var(&mut self, var: &mut Variable) {
    if self.pre_on_variable(var) {
      match var.permission.clone() {
        Some(p) => self.check_permission(var, p),
        None => var.permission = Some(self.perm_context)
      }
    }
  }

  fn visit_stmt(&mut self, child: &mut Stmt) {
    self.context_span = child.span;
    walk_stmt_mut(self, child)
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
}
