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

pub mod grammar;
pub mod let_lifting;

use self::grammar::*;
use self::let_lifting::*;
use ast::Program;
use partial::*;
use oak_runtime::*;

pub fn parse_bonsai(input: String) -> Partial<Program> {
  let state = bonsai::parse_program(input.into_state());
  let ast = match state.into_result() {
    ParseResult::Success(program) => Partial::Value(program),
    ParseResult::Partial(_, expectation)
  | ParseResult::Failure(expectation) => {
      println!("{:?}", expectation);
      Partial::Nothing
    }
  };
  ast.map(let_lifting)
}
