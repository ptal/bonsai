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
use session::*;
use middle::causality::symbolic_execution::State;
use middle::ir::guarded_command::*;
use middle::ir::scheduling::*;

pub type AllInstants = Vec<Instant>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instant {
  pub locations: State,
  pub program: Stmt,
  pub schedule_paths: Vec<Scheduling>,
}

impl Instant
{
  pub fn init(locations: State, program: Stmt) -> Self {
    Self::new(locations, program, vec![])
  }

  pub fn new(locations: State, program: Stmt, schedule_paths: Vec<Scheduling>) -> Self {
    Instant { locations, program, schedule_paths }
  }
}

pub struct Compiler {
  session: Session,
  context: Context,
  instants: AllInstants,
  ir: IR
}

impl Compiler {
  pub fn new(session: Session, context: Context, instants: AllInstants) -> Self {
    Compiler { session, context, instants, ir: IR::new() }
  }

  pub fn compile(self) -> Env<(Context, IR)> {
    Env::value(self.session, (self.context, self.ir))
  }
}
