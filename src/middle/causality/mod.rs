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

mod indexing;
mod causal_stmt;
mod causal_deps;
mod causal_model;
mod model_parameters;
mod solver;
mod symbolic_execution;

use context::*;
use session::*;
use middle::causality::indexing::*;
use middle::causality::solver::*;
use middle::causality::causal_stmt::*;
use middle::causality::symbolic_execution::*;
use middle::causality::model_parameters::*;

pub fn causality_analysis(session: Session, context: Context) -> Env<Context> {
  Env::value(session, context)
    .and_then(index_ops_and_delay)
    .and_then(execute_symbolically)
}

fn execute_symbolically(session: Session, (context, params): (Context, ModelParameters)) -> Env<Context> {
  let symbolic = SymbolicExecution::new(session, context);
  // let params = c.1;
  symbolic.for_each(|env| {
    env.and_then(|session, (context, stmt)|
          build_causal_model(session, context, stmt, params.clone()))
       .and_then(solve_causal_model)
    })
}
