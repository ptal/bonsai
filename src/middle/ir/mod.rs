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
use middle::ir::compiler::*;
pub use middle::ir::guarded_command::IR;

pub mod guarded_command;
pub mod compiler;
pub mod scheduling;

pub fn compile_to_guarded_commands(session: Session, (context, instants): (Context, AllInstants)) -> Env<(Context, IR)> {
  let compiler = Compiler::new(session, context, &instants);
  compiler.compile(instants)
}
