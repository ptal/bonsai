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

use context::*;
use std::cmp::max;

pub fn stream_bound<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  let stream_bound = StreamBound::new(context);
  stream_bound.compute()
}

struct StreamBound<'a> {
  context: Context<'a>
}

impl<'a> StreamBound<'a> {
  pub fn new(context: Context<'a>) -> Self {
    StreamBound {
      context: context
    }
  }

  fn compute(mut self) -> Partial<Context<'a>> {
    let bcrate_clone = self.context.clone_ast();
    self.visit_crate(bcrate_clone);
    Partial::Value(self.context)
  }

  fn bound_of<'b>(&'b mut self, var: String) -> &'b mut usize {
    self.context.stream_bound.entry(var).or_insert(0)
  }
}

impl<'a> Visitor<JClass> for StreamBound<'a>
{
  fn visit_binding(&mut self, binding: Binding) {
    self.bound_of(binding.name.unwrap());
  }

  fn visit_var(&mut self, var: Variable) {
    let bound = self.bound_of(var.name().unwrap());
    *bound = max(*bound, var.past);
  }
}
