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

pub mod functionalize_module;
mod duplicate;
mod matching_channel;
mod stream_bound;

use ast::*;
pub use middle::functionalize_module::*;
use middle::duplicate::*;
use middle::matching_channel::*;
use middle::stream_bound::*;
use partial::*;
use session::*;

pub fn analyse_bonsai(session: &Session, jcrate: JCrate) -> Partial<JCrate> {
  Partial::Value(jcrate)
    .and_then(|jcrate| duplicate(session, jcrate))
    .and_then(|jcrate| matching_channel(session, jcrate))
    .and_then(|jcrate| stream_bound(jcrate))
}
