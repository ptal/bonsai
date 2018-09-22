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

use middle::causality::causal_model::*;
use pcp::search::*;
use gcollections::ops::Bounded;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Scheduling {
  pub schedule: Vec<usize>
}

impl Scheduling
{
  /// We extract the order of the operation in the solved `space` in order to create a schedule.
  /// `schedule` is the order of the operations in one execution path.
  pub fn new(model: CausalModel, space: FDSpace) -> Self {
    let activated = model.params.activated;
    let mut op_order: Vec<_> =
      model.order_of_op.into_iter()
        .enumerate()
        .filter(|(op,_)| activated[*op])
        .map(|(op, order)| (op, order.read(&space.vstore).lower()))
        .collect();
    op_order.sort_unstable_by_key(|&v| v.1);
    Scheduling {
      schedule: op_order.into_iter().map(|v| v.0).collect()
    }
  }
}