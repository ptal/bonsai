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

use context::*;
use std::collections::HashMap;

pub type GuardedProgram = Vec<GuardedCommand>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IR {
  pub processes: HashMap<ProcessUID, GuardedProgram>,
}

impl IR {
  pub fn new() -> Self {
    IR { processes: HashMap::new() }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardedCommand {
  pub guard: Expr,
  pub action: Action
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action {
  Call(MethodCall),
  Init(Binding),
  LocalDrop(VarPath),
  Tell(Variable, Expr),
  Delay(VarPath, DelayKind),
  Push(VarPath),
  Prune(VarPath),
}
