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

/// For each variable, depending on its context, we approximate its permission.
/// Permissions can either be `Read`, `Write` or `ReadWrite` if the variable is read-only, write-only or both.
/// When calling external method, we do not know how the method is actually modifying the variable so we approximate it with `ReadWrite`.
/// In a tell statement `x <- e`, `x` is only wrote and variables occurring in `e` are supposed to be only read.
/// In an entailment condition `e |= e'`, every variables appearing in `e` or `e'` are supposed to be only read.

use context::*;
use session::*;

pub fn approximate_permission(session: Session, context: Context) -> Env<Context> {
  let permission = ApproximatePermission::new(session, context);
  permission.compute()
}

struct ApproximatePermission {
  session: Session,
  context: Context,
  perm_context: Permission,
  context_span: Span
}

impl ApproximatePermission {
  pub fn new(session: Session, context: Context) -> Self {
    ApproximatePermission {
      session: session,
      context: context,
      perm_context: Permission::ReadWrite,
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

  fn pre_on_variable(&self, var: &Variable) {
    if var.past > 0 {
      let uid = var.last_uid();
      match self.context.var_by_uid(uid).kind {
        Kind::Product => self.err_forbid_pre_on(var, "module"),
        Kind::Host => self.err_forbid_pre_on(var, "host"),
        Kind::Spacetime(Spacetime::SingleTime) => self.err_forbid_pre_on(var, "single_time"),
        _ => {
          if self.perm_context != Permission::Read {
            self.err_forbid_write_on_pre(var);
          }
        }
      }
    }
  }

  fn err_forbid_write_on_pre(&self, var: &Variable) {
    self.session().struct_span_err_with_code(var.span,
      &format!("forbidden write on `pre` variable."),
      "E0016")
    .span_label(var.span, &format!("write here"))
    .help(&"`pre` variables can only be read. External function call's parameters are considered as read/write variables.")
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
}

impl VisitorMut<JClass> for ApproximatePermission
{
  fn visit_var(&mut self, var: &mut Variable) {
    self.pre_on_variable(var);
    var.permission = self.perm_context;
  }

  fn visit_stmt(&mut self, child: &mut Stmt) {
    self.context_span = child.span;
    walk_stmt_mut(self, child)
  }

  fn visit_tell(&mut self, var: &mut Variable, expr: &mut Expr) {
    let old = self.perm_context;
    self.perm_context = Permission::Write;
    self.visit_var(var);
    self.perm_context = Permission::Read;
    self.visit_expr(expr);
    self.perm_context = old;
  }

  fn visit_entailment(&mut self, rel: &mut EntailmentRel) {
    let old = self.perm_context;
    self.perm_context = Permission::Read;
    self.visit_var(&mut rel.left);
    self.visit_expr(&mut rel.right);
    self.perm_context = old;
  }
}
