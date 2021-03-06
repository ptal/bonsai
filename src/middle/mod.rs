// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod duplicate;
mod undeclared;
mod resolve;
mod initialization;
mod stream_bound;
mod infer_permission;
mod constructor;
mod instantaneous;
mod causality;
mod recursive_call;
mod search_tree_wf;
mod rewrite_reincarnation;
mod collect_module_in_proc;

use context::*;
use session::*;
use middle::duplicate::*;
use middle::undeclared::*;
use middle::resolve::*;
use middle::initialization::*;
use middle::stream_bound::*;
use middle::infer_permission::*;
use middle::constructor::*;
use middle::instantaneous::*;
use middle::causality::*;
use middle::recursive_call::*;
use middle::search_tree_wf::*;
use middle::rewrite_reincarnation::*;
use middle::collect_module_in_proc::*;

pub fn analyse_bonsai(env: Env<Context>) -> Env<Context> {
  env
    .and_next(rewrite_reincarnation)
    .and_then(duplicate)
    .and_then(undeclared)
    .and_then(resolve)
    .and_then(constructor)
    .and_then(initialization)
    .and_then(stream_bound)
    .and_then(infer_permission)
    .and_then(recursive_call)
    .and_next(instantaneous_analysis)
    .and_next(search_tree_wf)
    .and_next(causality_analysis)
    .and_next(collect_module_in_proc)
}
