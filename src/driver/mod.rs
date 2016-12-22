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

pub mod config;
mod module_file;
mod project;

pub use self::config::*;
use self::project::*;
use partial::*;
use front;
use middle;
use back;

pub fn run() {
  let config = Config::new();
  let project = Project::new(&config);

  for module in project {
    Partial::Value(module.input_as_string())
    .and_then(front::parse_bonsai)
    .and_then(middle::analyse_bonsai)
    .and_then(|m| back::generate_chococubes(m, &config))
    .map(|output| module.write_output(output));
  }
}

