// Copyright 2018 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Given a process P, we index every access operation with an integer.
/// For example, `read x` becomes `read^n x` where `n` is its index (field `op_no` in `Variable`).

use context::*;
use session::*;

pub fn index_op(session: Session, context: Context) -> Env<Context> {
  let index = IndexOp::new(session, context);
  index.compute()
}

struct IndexOp {
  session: Session,
  context: Context,
  index_gen: usize
}

impl IndexOp {
  pub fn new(session: Session, context: Context) -> Self {
    IndexOp { session, context, index_gen:0 }
  }

  fn compute(mut self) -> Env<Context> {
    let mut bcrate_clone = self.context.clone_ast();
    self.visit_crate(&mut bcrate_clone);
    self.context.replace_ast(bcrate_clone);
    Env::value(self.session, self.context)
  }

  fn gen_op_no(&mut self) -> usize {
    let op_no = self.index_gen;
    self.index_gen += 1;
    op_no
  }
}

impl VisitorMut<JClass> for IndexOp
{
  fn visit_var(&mut self, var: &mut Variable) {
    var.op_no = self.gen_op_no();
  }
}
