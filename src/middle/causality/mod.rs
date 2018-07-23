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

mod index_op;
mod causal_stmt;
mod causal_deps;
mod causal_model;
mod model_parameters;
mod solver;

use context::*;
use session::*;
use middle::causality::index_op::*;
use middle::causality::causal_stmt::*;
use middle::causality::solver::*;

pub fn causality_analysis(session: Session, context: Context) -> Env<Context> {
  Env::value(session, context)
    .and_then(index_op)
    .and_then(build_causal_model)
    .and_then(solve_causal_model)
}
