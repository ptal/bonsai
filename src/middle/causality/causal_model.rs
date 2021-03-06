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

/// Constraint model modelling the causal dependencies of an execution path in a spacetime program.
/// The causal dependencies of the full program can modelled with `Vec<CausalModel>` (see `causal_stmt.rs`).
/// It is described in the Section 4.5 of the dissertation (Talbot, 2018).

use middle::causality::model_parameters::*;
use pcp::search::*;
use pcp::concept::*;
use pcp::propagators::*;
use gcollections::ops::*;
use interval::interval_set::*;
use interval::ops::Range;
use std::clone::Clone;

pub struct CausalModel {
  pub space: FDSpace,
  pub latest_ops: Vec<usize>,
  pub instantaneous: bool,
  pub order_of_op: Vec<Var<VStore>>,
  pub params: ModelParameters,
}

impl CausalModel {
  pub fn new(params: ModelParameters) -> Self {
    let mut m = CausalModel {
      space: FDSpace::empty(),
      latest_ops: vec![],
      instantaneous: true,
      order_of_op: vec![],
      params
    };
    m.init_op_vars();
    m
  }

  fn init_op_vars(&mut self) {
    let n =  self.num_ops();
    for _ in 0..n {
      let v = Box::new(self.space.vstore.alloc(IntervalSet::new(0, (n-1) as i32))) as Var<VStore>;
      self.order_of_op.push(v);
    }
  }

  pub fn num_ops(&self) -> usize {
    self.params.num_ops()
  }

  /// We join the constraint stores of `self` and `other` with the following assumptions:
  ///   1. The variable stores are identical.
  ///   2. The constraint stores are completely different.
  ///   3. The model parameters can be joined (see `ModelParameters::join`).
  /// The second assumption implies that we do not check for identical constraints.
  pub fn join_constraints(mut self, other: CausalModel) -> CausalModel
  {
    assert_eq!(self.space.vstore.size(), other.space.vstore.size(),
      "join_constraints: The variables store must be identical.");
    self.latest_ops.extend(other.latest_ops.into_iter());
    // NOTE: the conjunctive parallel statement <> is a weak preemption so during an instant, it behaves like ||.
    self.instantaneous = self.instantaneous && other.instantaneous;
    let cstore = other.space.cstore;
    for i in 0..cstore.size() {
      self.space.cstore.alloc(cstore[i].bclone());
    }
    self.params = self.params.join(other.params);
    self
  }

  pub fn cartesian_product(left: Vec<CausalModel>, right: Vec<CausalModel>)
   -> Vec<CausalModel>
  {
    let mut res = vec![];
    for s1 in left {
      for s2 in right.clone() {
        res.push(s1.clone().join_constraints(s2));
      }
    }
    res
  }

  pub fn fold(self, models: Vec<CausalModel>) -> CausalModel {
    models.into_iter().fold(self, |a, m| a.join_constraints(m))
  }

  pub fn add_simultaneous_ops_constraint(&mut self, ops: Vec<usize>) {
    if ops.len() > 1 {
      let vars: Vec<Var<VStore>> = ops.into_iter().map(|op| self.order_of_op[op].bclone()).collect();
      let all_equal = Box::new(AllEqual::new(vars));
      self.space.cstore.alloc(all_equal);
    }
  }

  pub fn add_after_latest_constraint(&mut self, after_op: usize) {
    for before_op in self.latest_ops.clone() {
      self.add_sequential_constraint(before_op, after_op);
    }
    self.latest_ops = vec![after_op];
  }

  pub fn add_sequential_constraint(&mut self, before_op: usize, after_op: usize) {
    let gt = Box::new(x_greater_y(
      self.order_of_op[after_op].bclone(), self.order_of_op[before_op].bclone()));
    self.space.cstore.alloc(gt);
  }
}

impl Clone for CausalModel {
  fn clone(&self) -> Self {
    CausalModel {
      space: FDSpace::new(self.space.vstore.clone(), self.space.cstore.clone()),
      latest_ops: self.latest_ops.clone(),
      instantaneous: self.instantaneous,
      order_of_op: self.order_of_op.iter().map(|v| v.bclone()).collect(),
      params: self.params.clone()
    }
  }
}
