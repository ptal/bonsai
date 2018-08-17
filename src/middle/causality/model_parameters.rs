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
  /// We keep monotonic read operations encountered in an expression.
  /// This is useful for conditional statements that should not add read/write constraints on these variables.
  pub monotonic_read_ops: Vec<usize>,
  /// We store the operations couple that are not subject to read/write constraints.
  /// For example: when a(1) |= b then a(2) <- 1 end, `a(1)` and `a(2)` must be not be constrained (a write can happen after a read in this context).
  pub relaxed_rw_ops: Vec<(usize, usize)>,
  pub activated: Vec<bool>,
}

impl ModelParameters {
  pub fn new() -> Self {
    ModelParameters {
      var_of_op: vec![],
      monotonic_read_ops: vec![],
      relaxed_rw_ops: vec![],
      activated: vec![],
    }
  }

  pub fn num_ops(&self) -> usize {
    self.var_of_op.len()
  }

  pub fn alloc_variable(&mut self, var: &mut Variable) {
    var.op_no = self.num_ops();
    self.var_of_op.push(var.clone());
    self.activated.push(false);
  }

  pub fn activate_op(&mut self, op: usize) {
    self.activated[op] = true;
  }

  /// Preconditions:
  ///   1. Both model parameters have the same number of operations.
  ///   2. `var_of_op` are the same.
  pub fn join(mut self, mut other: ModelParameters) -> ModelParameters
  {
    assert_eq!(self.num_ops(), other.num_ops(),
      "join: model parameters must have the same number of operations.");
    assert_eq!(self.var_of_op, other.var_of_op,
      "join: `var_of_op` must be identical.");
    self.relaxed_rw_ops.append(&mut other.relaxed_rw_ops);
    self.monotonic_read_ops.append(&mut other.monotonic_read_ops);
    for i in 0..self.activated.len() {
      self.activated[i] = self.activated[i] || other.activated[i];
    }
    self
  }

  pub fn store_monotonic_read(&mut self, op: usize) {
    self.monotonic_read_ops.push(op);
  }

  pub fn register_relaxed_op(&mut self, op: usize) {
    debug!("in register_relaxed_op: {:?}", self.monotonic_read_ops);
    debug!("in register_relaxed_op: op: {}", self.var_of_op[op]);
    for relaxed_op in &self.monotonic_read_ops {
      debug!("in register_relaxed_op: relaxed_op: {}", self.var_of_op[*relaxed_op]);
      if self.var_of_op[*relaxed_op].last_uid() == self.var_of_op[op].last_uid() {
        self.relaxed_rw_ops.push((*relaxed_op, op));
        debug!("Add relaxed ops {} / {}.", *relaxed_op, op);
      }
    }
  }

  pub fn is_rw_constrained(&self, op1: usize, op2: usize) -> bool {
    !(self.relaxed_rw_ops.contains(&(op1, op2)) || self.relaxed_rw_ops.contains(&(op2, op1)))
  }
}
