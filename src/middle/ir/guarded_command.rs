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
use middle::causality::symbolic_execution::State;
use middle::ir::compiler::AllInstants;
use std::collections::HashMap;
use std::cmp;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardedCommandFactory {
  locations: Vec<Variable>,
}

impl GuardedCommandFactory
{
  pub fn new(all_instants: &AllInstants) -> Self {
    let mut factory = GuardedCommandFactory{ locations: vec![] };
    factory.initialize_locations_names(all_instants);
    factory
  }

  fn initialize_locations_names(&mut self, all_instants: &AllInstants) {
    let max_loc = all_instants.values()
      .flat_map(|p| p.iter())
      .flat_map(|i| i.locations.iter())
      .cloned()
      .fold(0, cmp::max);
    for loc in 0..max_loc+1 {
      let loc_path = VarPath::gen(&format!("loc{}",loc));
      let var = Variable::access(DUMMY_SP, loc_path, None);
      self.locations.push(var);
    }
  }

  pub fn create_locations_guard(&self, locations: State) -> Box<Expr> {
    let guard = self.make_expr(ExprKind::Trilean(SKleene::True));
    locations.into_iter().fold(guard, |a, l| {
      let loc_var = self.locations[l].clone();
      self.make_expr(ExprKind::And(a,self.make_expr(ExprKind::Var(loc_var))))
    })
  }

  fn make_expr(&self, expr_kind: ExprKind) -> Box<Expr> {
    Box::new(Expr::new(DUMMY_SP, expr_kind))
  }
}
