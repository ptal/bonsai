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

pub fn approximate_permission<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  let permission = ApproximatePermission::new(context);
  permission.compute()
}

struct ApproximatePermission<'a> {
  context: Context<'a>,
  perm_context: Permission
}

impl<'a> ApproximatePermission<'a> {
  pub fn new(context: Context<'a>) -> Self {
    ApproximatePermission {
      context: context,
      perm_context: Permission::ReadWrite,
    }
  }

  fn compute(mut self) -> Partial<Context<'a>> {
    let mut bcrate_clone = self.context.clone_ast();
    self.visit_crate(&mut bcrate_clone);
    self.context.replace_ast(bcrate_clone);
    Partial::Value(self.context)
  }
}

impl<'a> VisitorMut<JClass> for ApproximatePermission<'a>
{
  fn visit_var(&mut self, var: &mut Variable) {
    var.permission = self.perm_context;
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
