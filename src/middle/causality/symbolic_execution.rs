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

/// Given a process P, we iterate over all the instants of P, and all the possible execution paths of these instants.

use context::*;
use session::*;
use gcollections::VectorStack;
use gcollections::ops::*;
use std::collections::{HashSet};

/// A state is the set of all delay statements that must be resumed in the next instant.
/// Therefore, `usize` are only pushed by parallel statements.
/// An empty `HashSet` represents a terminated execution path or the first instant of a process.
pub type State = HashSet<usize>;

/// The set of all states such that one state represents one possible next instant.
#[derive(Debug)]
struct StatesSet {
  /// Invariant: `states` is never empty.
  /// An element in the vector represents the possible paths of execution leading to distinct next instant.
  states: Vec<State>
}

impl StatesSet {
  fn terminated_state() -> Self {
    StatesSet { states: vec![HashSet::new()] }
  }

  fn paused_state(num_state: usize) -> Self {
    let mut state = HashSet::new();
    state.insert(num_state);
    StatesSet { states: vec![state] }
  }

  fn terminated_index(&self) -> Option<usize> {
    for (i, state) in self.states.iter().enumerate() {
      if state.is_empty() {
        return Some(i)
      }
    }
    None
  }

  fn remove_terminated(&mut self) -> bool {
    let i = self.terminated_index();
    if let Some(i) = i {
      self.states.swap_remove(i);
      true
    }
    else {
      false
    }
  }

  /// If one execution path is terminated, we remove it and add all `next_states`.
  /// If `next_states` was added, we return true, otherwise false.
  fn followed_by(&mut self, mut next_states: StatesSet) -> bool {
    let one_path_terminated = self.remove_terminated();
    if one_path_terminated {
      self.states.append(&mut next_states.states);
      true
    }
    else {
      false
    }
  }

  fn join(&mut self, mut other: StatesSet) {
    let t1 = self.terminated_index();
    let t2 = other.terminated_index();
    if t1.is_some() && t2.is_some() {
      self.remove_terminated();
    }
    self.states.append(&mut other.states);
  }

  fn next_states(mut self) -> Vec<State> {
    self.remove_terminated();
    self.states
  }

  fn is_instantaneous(&self) -> bool {
    self.states.len() == 1 && self.states[0].is_empty()
  }

  fn cartesian_product<F>(self, other: StatesSet, term_join: F) -> Self
    where F: Fn(bool, bool) -> bool
  {
    let mut set = Self::terminated_state();
    for s1 in self.states {
      for s2 in other.states.clone() {
        let is_terminated = term_join(s1.is_empty(), s2.is_empty());
        let union =
          if is_terminated {
            HashSet::new()
          }
          else {
            s1.union(&s2).cloned().collect()
          };
        if !set.contains(&union) {
          set.states.push(union);
        }
      }
    }
    set
  }

  fn contains(&self, state: &State) -> bool {
    self.states.iter().any(|s| s == state)
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instant {
  pub locations: State,
  pub program: Stmt
}

impl Instant
{
  pub fn new(locations: State, program: Stmt) -> Self {
    Instant { locations, program }
  }
}


#[derive(Clone, Debug, PartialEq, Eq)]
enum ResidualStmt {
  Terminated,
  Paused,
  Next(Stmt)
}

pub struct SymbolicExecution {
  session: Session,
  context: Context,
  visited_states: Vec<State>,
  next_instants: VectorStack<Instant>
}

impl SymbolicExecution
{
  fn new(session: Session, context: Context) -> Self {
    SymbolicExecution {
      session: session,
      context: context,
      visited_states: vec![],
      next_instants: VectorStack::empty()
    }
  }

  pub fn for_each_instant<F>(mut session: Session, mut context: Context, f: F) -> Env<Context>
   where F: Clone + Fn(Env<(Context, Stmt)>) -> Env<Context>
  {
    let mut fake = false;
    for uid in context.entry_points.clone() {
      let mut this = SymbolicExecution::new(session, context);
      this.push_process(uid.clone());
      let env = this.for_each(f.clone());
      let (s, data) = env.decompose();
      fake = fake || data.is_fake();
      match data {
        Partial::Value(c)
      | Partial::Fake(c) => {
          context = c;
          session = s;
        }
        _ => { return Env::nothing(s) }
      }
    }
    if fake { Env::fake(session, context) }
    else { Env::value(session, context)}
  }

  fn push_process(&mut self, uid: ProcessUID) {
    let process = self.context.find_proc(uid.clone());
    let state = HashSet::new();
    self.push_instant(Some(process.body), state);
  }

  fn for_each<F>(mut self, f: F) -> Env<Context>
   where F: Fn(Env<(Context, Stmt)>) -> Env<Context>
  {
    let mut fake = false;
    while let Some(instant) = self.next() {
      let env = f(Env::value(self.session, (self.context, instant.program.clone())));
      let (session, data) = env.decompose();
      fake = fake || data.is_fake();
      match data {
        Partial::Value(context)
      | Partial::Fake(context) => {
          self.context = context;
          self.session = session;
        }
        _ => { return Env::nothing(session) }
      }
    }
    if fake { Env::fake(self.session, self.context) }
    else { Env::value(self.session, self.context)}
  }

  /// Returns `true` if the state has not been visited before.
  fn already_visited(&self, state: State) -> bool {
    self.visited_states.iter().any(|s| s == &state)
  }

  fn push_instant(&mut self, next_program: Option<Stmt>, state: State) {
    self.visited_states.push(state.clone());
    let nothing = Stmt::new(DUMMY_SP, StmtKind::Nothing);
    let instant = Instant::new(state, next_program.clone().unwrap_or(nothing));
    self.next_instants.push(instant);
  }

  fn next(&mut self) -> Option<Instant> {
    trace!("current number of next instants: {}", self.next_instants.len());
    let instant = self.next_instants.pop();
    if let Some(instant) = instant.clone() {
      self.compute_residual(instant.program.clone());
      trace!("after residual: {}", self.next_instants.len());
    }
    instant
  }

  /// We first compute all the distinct set of locations states in which `current` could stop.
  /// Then, for each possible set of locations, we compute its residual statement.
  fn compute_residual(&mut self, current: Stmt) {
    let states_set = self.next_states_stmt(current.clone());
    for state in states_set.next_states() {
      if !self.already_visited(state.clone()) {
        let residual = self.reduce_stmt(current.clone(), state.clone());
        let instant = match residual {
          ResidualStmt::Terminated => None,
          ResidualStmt::Paused => None, // It means that the next instant is `nothing`.
          ResidualStmt::Next(next) => Some(next)
        };
        self.push_instant(instant, state);
      }
    }
  }

  fn term_and(a: bool, b: bool) -> bool { a && b }
  fn term_or(a: bool, b: bool) -> bool { a || b }

  fn next_states_stmt(&self, stmt: Stmt) -> StatesSet
  {
    use ast::StmtKind::*;
    match stmt.node {
      DelayStmt(delay) => self.next_states_delay(delay),
      Let(body) => self.next_states_let(body),
      Seq(branches) => self.next_states_seq(branches),
      When(cond, then_branch, else_branch) =>
        self.next_states_when(cond, *then_branch, *else_branch),
      OrPar(branches) => self.next_states_par(branches, Self::term_or),
      AndPar(branches) => self.next_states_par(branches, Self::term_and),
      Loop(body) => self.next_states_loop(*body),
      Universe(queue, body) => self.next_states_universe(queue, *body),
      QFUniverse(body) => self.next_states_qf_universe(*body),
      Space(_)
    | Prune
    | LocalDrop(_)
    | Nothing
    | ExprStmt(_)
    | Tell(_, _) => StatesSet::terminated_state(),
      _ => StatesSet::terminated_state(),
      // Suspend(cond, body) => self.next_states_suspend(cond, *body, model),
      // Abort(cond, body) => self.next_states_abort(cond, *body, model),
      // ProcCall(var, process, args) => self.next_states_proc_call(var, process, args),
    }
  }

  fn next_states_delay(&self, delay: Delay) -> StatesSet
  {
    StatesSet::paused_state(delay.state_num)
  }

  fn next_states_seq(&self, children: Vec<Stmt>) -> StatesSet
  {
    let mut states = StatesSet::terminated_state();
    for child in children {
      let next = self.next_states_stmt(child);
      if !states.followed_by(next) {
        return states;
      }
    }
    states
  }

  fn next_states_let(&self, let_stmt: LetStmt) -> StatesSet
  {
    self.next_states_stmt(*(let_stmt.body))
  }

  fn next_states_when(&self, _c: Expr, then_branch: Stmt, else_branch: Stmt) -> StatesSet
  {
    let mut states_then = self.next_states_stmt(then_branch);
    let states_else = self.next_states_stmt(else_branch);
    states_then.join(states_else);
    states_then
  }

  fn next_states_par<F>(&self, children: Vec<Stmt>, term_join: F) -> StatesSet
    where F: Clone + Fn(bool, bool) -> bool
  {
    let mut next_states: Vec<_> = children.into_iter()
      .map(|child| self.next_states_stmt(child))
      .collect();
    let mut next = next_states.remove(0);
    for states_set in next_states {
      next = next.cartesian_product(states_set, term_join.clone());
    }
    next
  }

  fn next_states_loop(&self, body: Stmt) -> StatesSet
  {
    self.next_states_stmt(body)
  }

  fn next_states_universe(&self, _queue: Variable, body: Stmt) -> StatesSet
  {
    self.next_states_stmt(body)
  }

  fn next_states_qf_universe(&self, body: Stmt) -> StatesSet
  {
    self.next_states_stmt(body)
  }

  fn reduce_stmt(&self, stmt: Stmt, state: State) -> ResidualStmt
  {
    use ast::StmtKind::*;
    let span = stmt.span;
    match stmt.node {
      DelayStmt(delay) => self.reduce_delay(delay, state),
      Let(stmt) => self.reduce_let(span, stmt, state),
      Seq(branches) => self.reduce_seq(branches, state),
      When(cond, then_branch, else_branch) =>
        self.reduce_when(cond, *then_branch, *else_branch, state),
      OrPar(branches) => self.reduce_or_par(span, branches, state),
      AndPar(branches) => self.reduce_and_par(span, branches, state),
      Loop(body) => self.reduce_loop(*body, state),
      Universe(queue, body) => self.reduce_universe(queue, *body, state),
      QFUniverse(body) => self.reduce_qf_universe(*body, state),
      Space(_)
    | Prune
    | LocalDrop(_)
    | Nothing
    | ExprStmt(_)
    | Tell(_, _) => ResidualStmt::Terminated,
      _ => ResidualStmt::Terminated
      // Suspend(cond, body) => self.reduce_suspend(cond, *body, model, state),
      // Abort(cond, body) => self.reduce_abort(cond, *body, model, state),
      // ProcCall(var, process, args) => self.reduce_proc_call(var, process, args),
    }
  }

  fn reduce_delay(&self, delay: Delay, state: State) -> ResidualStmt
  {
    if state.contains(&delay.state_num) {
      ResidualStmt::Paused
    }
    else {
      ResidualStmt::Terminated
    }
  }

  fn reduce_seq(&self, children: Vec<Stmt>, state: State) -> ResidualStmt
  {
    let mut next_stmts = vec![];
    let mut has_paused = false;
    for child in children {
      if has_paused {
        next_stmts.push(child);
      }
      else {
        let next = self.reduce_stmt(child, state.clone());
        match next {
          ResidualStmt::Terminated => (),
          ResidualStmt::Paused => has_paused = true,
          ResidualStmt::Next(next) => {
            has_paused = true;
            next_stmts.push(next);
          }
        }
      }
    }
    self.rebuild_seq(next_stmts, has_paused)
  }

  fn rebuild_seq(&self, next_stmts: Vec<Stmt>, has_paused: bool) -> ResidualStmt {
    if next_stmts.is_empty() {
      if has_paused {
        ResidualStmt::Paused
      }
      else {
        ResidualStmt::Terminated
      }
    }
    else {
      let sp = mk_sp(next_stmts.first().unwrap().span.lo, next_stmts.last().unwrap().span.hi);
      ResidualStmt::Next(Stmt::new(sp, StmtKind::Seq(next_stmts)))
    }
  }

  fn reduce_let(&self, span: Span, mut let_stmt: LetStmt, state: State) -> ResidualStmt
  {
    let kind = let_stmt.kind();
    let next = self.reduce_stmt(*(let_stmt.body), state);
    match next {
      ResidualStmt::Next(next) => {
        // Host, single_space and world_line variables are only initialized during their first instant.
        match kind {
          Kind::Host
        | Kind::Spacetime(Spacetime::SingleSpace)
        | Kind::Spacetime(Spacetime::WorldLine) => ResidualStmt::Next(next),
          _ => {
            let_stmt.body = Box::new(next);
            ResidualStmt::Next(Stmt::new(span, StmtKind::Let(let_stmt)))
          }
        }
      }
      n => n
    }
  }

  fn reduce_when(&self, _condition: Expr, then_branch: Stmt, else_branch: Stmt,
      state: State) -> ResidualStmt
  {
    let next_then = self.reduce_stmt(then_branch, state.clone());
    let next_else = self.reduce_stmt(else_branch, state);
    if next_then != ResidualStmt::Terminated {
      next_then
    }
    else if next_else != ResidualStmt::Terminated {
      next_else
    }
    else {
      ResidualStmt::Terminated
    }
  }

  fn reduce_or_par(&self, span: Span, children: Vec<Stmt>, state: State) -> ResidualStmt {
    self.reduce_par(span, children, state, |next| StmtKind::OrPar(next))
  }

  fn reduce_and_par(&self, span: Span, children: Vec<Stmt>, state: State) -> ResidualStmt {
    self.reduce_par(span, children, state, |next| StmtKind::AndPar(next))
  }

  fn reduce_par<F>(&self, span: Span, children: Vec<Stmt>, state: State, build_par: F) -> ResidualStmt
    where F: Fn(Vec<Stmt>) -> StmtKind
  {
    let reduced: Vec<_> = children.into_iter()
      .map(|c| self.reduce_stmt(c, state.clone()))
      .filter(|r| r != &ResidualStmt::Terminated)
      .collect();
    if reduced.is_empty() {
      ResidualStmt::Terminated
    }
    else {
      let next: Vec<_> = reduced.into_iter()
        .filter_map(|r| if let ResidualStmt::Next(n) = r { Some(n) } else { None })
        .collect();
      if next.is_empty() {
        ResidualStmt::Paused
      }
      else {
        ResidualStmt::Next(
          Stmt::new(span, build_par(next))
        )
      }
    }
  }

  fn reduce_loop(&self, body: Stmt, state: State) -> ResidualStmt
  {
    use middle::causality::symbolic_execution::ResidualStmt::*;
    let next_body = self.reduce_stmt(body.clone(), state.clone());
    match next_body {
      // No instruction pointer currently inside the loop.
      Terminated => Terminated,
      // We go back to the beginning of the loop.
      Paused => Next(body),
      // We rewrite `P` into `P; loop P end`.
      Next(stmt) => {
        let sp = stmt.span;
        Next(Stmt::new(sp, StmtKind::Seq(vec![stmt, Stmt::new(sp, StmtKind::Loop(Box::new(body)))])))
      }
    }
  }

  fn reduce_universe(&self, _queue: Variable, body: Stmt, state: State) -> ResidualStmt
  {
    self.reduce_stmt(body, state)
  }

  fn reduce_qf_universe(&self, body: Stmt, state: State) -> ResidualStmt
  {
    self.reduce_stmt(body, state)
  }
}
