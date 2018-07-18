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

/// Parameters of the causality model described in the Section 4.5.2 of the dissertation (Talbot, 2018).

use context::*;
use std::clone::Clone;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ModelParameters {
  pub var_of_op: Vec<Variable>,
  pub activated: Vec<bool>,
}

impl ModelParameters {
  pub fn new() -> Self {
    ModelParameters {
      var_of_op: vec![],
      activated: vec![]
    }
  }

  pub fn num_ops(&self) -> usize {
    self.activated.len()
  }

  pub fn alloc_variable(&mut self, var: &mut Variable) {
    var.op_no = self.num_ops();
    self.activated.push(false);
    self.var_of_op.push(var.clone());
  }

  /// Preconditions:
  ///   1. Both model parameters have the same number of operations.
  ///   2. `var_of_op` are the same.
  pub fn join(mut self, other: ModelParameters) -> ModelParameters
  {
    assert_eq!(self.num_ops(), other.num_ops(),
      "join: model parameters must have the same number of operations.");
    assert_eq!(self.var_of_op, other.var_of_op,
      "join: `var_of_op` must be identical.");
    for i in 0..self.activated.len() {
      self.activated[i] = self.activated[i] || other.activated[i];
    }
    self
  }

  pub fn activate_op(&mut self, op: usize) {
    self.activated[op] = true;
  }
}
