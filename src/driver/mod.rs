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
pub mod module_file;
mod project;

pub use self::config::*;
use self::project::*;
use partial::*;
use front;
use middle;
use back;
use jast::JCrate;

static ABORT_MSG: &'static str = "stop due to compilation errors";

pub fn run() {
  let config = Config::new();
  run_front(&config)
    .and_then(|jcrate| run_middle(&config, jcrate))
    .map(|jcrate| run_back(&config, jcrate));
}

fn run_front(config: &Config) -> Partial<JCrate> {
  let project = Project::new(config);
  let mut jcrate = JCrate::new();
  for file in project {
    Partial::Value(file.input_as_string())
      .and_then(front::parse_bonsai)
      .and_then(|ast| middle::functionalize_module(file, ast))
      .map(|module| jcrate.modules.push(module))
      .expect(ABORT_MSG);
  }
  Partial::Value(jcrate)
}

fn run_middle(_config: &Config, jcrate: JCrate) -> Partial<JCrate> {
  middle::analyse_bonsai(jcrate)
}

fn run_back(config: &Config, jcrate: JCrate) {
  for module in jcrate.modules {
    if !module.file.is_lib() {
      let file = module.file.clone();
      back::generate_runtime(module, &config)
        .map(|output| file.write_output(output))
        .expect(ABORT_MSG);
    }
  }
}