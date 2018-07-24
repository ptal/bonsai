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

/// Given a process P, we iterate over all the possible execution paths of a spacetime program.

use context::*;
use session::*;
use gcollections::VectorStack;
use gcollections::ops::*;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialOrd, Ord, Eq, PartialEq, Debug)]
enum CompletionCode {
  Terminate = 0,
  Pause = 1,
  PauseUp = 2,
  Stop = 3
}

#[derive(Clone, Debug)]
pub struct SymbolicInstant {
  pub program: Stmt,
  pub current_process: ProcessUID,
  pub state: HashSet<usize>
}

impl SymbolicInstant {
  pub fn new(program: Stmt, current_process: ProcessUID, state: HashSet<usize>) -> Self {
    SymbolicInstant { program, current_process, state }
  }
}

pub struct SymbolicExecution {
  session: Session,
  context: Context,
  visited_states: Vec<HashSet<usize>>,
  next_instants: VectorStack<SymbolicInstant>
}

impl SymbolicExecution {
  pub fn new(session: Session, context: Context) -> Self {
    SymbolicExecution {
      session: session,
      context: context,
      visited_states: vec![],
      next_instants: VectorStack::empty()
    }
  }

  pub fn for_each<F>(mut self, f: F) -> Env<Context>
   where F: Fn(Env<(Context, SymbolicInstant)>) -> Env<Context>
  {
    let mut fake = false;
    self.initialize();
    while let Some(instant) = self.next() {
      let env = f(Env::value(self.session, (self.context, instant)));
      let (session, context) = env.decompose();
      if context.is_value() || context.is_fake() {
        fake = fake || context.is_fake();
        self.context = context.unwrap_all();
        self.session = session;
      }
      else { return Env::nothing(session) }
    }
    if fake { Env::fake(self.session, self.context) }
    else { Env::value(self.session, self.context) }
  }

  fn push_instant(&mut self, instant: SymbolicInstant) {
    if !self.visited_states.iter().any(|s| s == &instant.state) {
      self.visited_states.push(instant.state.clone());
      self.next_instants.insert(instant);
    }
  }

  fn initialize(&mut self) {
    for (i, uid) in self.context.entry_points.clone().into_iter().enumerate() {
      let process = self.context.find_proc(uid.clone());
      let mut state = HashSet::new();
      state.insert(i);
      let instant = SymbolicInstant::new(process.body, uid, state);
      self.push_instant(instant);
    }
  }

  fn next(&mut self) -> Option<SymbolicInstant> {
    let instant = self.next_instants.pop();
    if let Some(instant) = instant.clone() {
      self.compute_residual(instant);
    }
    instant
  }

  fn compute_residual(&mut self, instant: SymbolicInstant) {

  }
}
