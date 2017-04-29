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
mod matching_ref;
mod stream_bound;
mod approximate_permission;

use context::*;
use middle::duplicate::*;
use middle::undeclared::*;
use middle::resolve::*;
use middle::matching_ref::*;
use middle::stream_bound::*;
use middle::approximate_permission::*;

pub fn analyse_bonsai<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  Partial::Value(context)
    .and_then(duplicate)
    .and_then(undeclared)
    .and_then(resolve)
    .and_then(matching_ref)
    .and_then(stream_bound)
    .and_then(approximate_permission)
}
