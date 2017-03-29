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

/// For each binding, we compute its maximum stream bound. It is the maximum number of `pre` occuring before the variable.

use ast::*;
use visitor::*;
use partial::*;
use std::cmp::max;

pub fn stream_bound<H: Clone>(bcrate: Crate<H>) -> Partial<Crate<H>> {
  let stream_bound = StreamBound::new(bcrate);
  stream_bound.compute()
}

struct StreamBound<H: Clone> {
  bcrate: Crate<H>
}

impl<H: Clone> StreamBound<H> {
  pub fn new(bcrate: Crate<H>) -> Self {
    StreamBound {
      bcrate: bcrate
    }
  }

  fn compute(mut self) -> Partial<Crate<H>> {
    let bcrate_clone = self.bcrate.clone();
    self.visit_crate(bcrate_clone);
    Partial::Value(self.bcrate)
  }

  fn visit_stream_var(&mut self, var: StreamVar) {
    let bound = self.bcrate.stream_bound.entry(var.name()).or_insert(0);
    *bound = max(*bound, var.past);
  }

  fn visit_expr(&mut self, expr: Expr) {
    use ast::ExprKind::*;
    match expr.node {
      JavaNew(_, args) => {
        for arg in args {
          self.visit_expr(arg);
        }
      }
      JavaObjectCall(_, methods) => {
        for method in methods {
          for arg in method.args {
            self.visit_expr(arg);
          }
        }
      }
      JavaThisCall(method) => {
        for arg in method.args {
          self.visit_expr(arg);
        }
      }
      Variable(var) => { self.visit_stream_var(var); }
      _ => ()
    }
  }
}

impl<H: Clone> Visitor<H, ()> for StreamBound<H> {
  unit_visitor_impl!(bcrate, H);
  unit_visitor_impl!(module, H);
  unit_visitor_impl!(sequence);
  unit_visitor_impl!(parallel);
  unit_visitor_impl!(space);
  unit_visitor_impl!(let_binding);
  unit_visitor_impl!(pause);
  unit_visitor_impl!(pause_up);
  unit_visitor_impl!(stop);
  unit_visitor_impl!(exit);
  unit_visitor_impl!(proc_call);
  unit_visitor_impl!(fn_call);
  unit_visitor_impl!(module_call);
  unit_visitor_impl!(nothing);

  fn visit_binding(&mut self, binding: LetBindingBase) {
    self.bcrate.stream_bound.entry(binding.name).or_insert(1);
  }

  fn visit_tell(&mut self, var: StreamVar, expr: Expr) {
    self.visit_stream_var(var);
    self.visit_expr(expr);
  }

  fn visit_when(&mut self, cond: Condition, child: Stmt) {
    let rel = cond.unwrap();
    self.visit_stream_var(rel.left);
    self.visit_expr(rel.right);
    self.visit_stmt(child)
  }
}
