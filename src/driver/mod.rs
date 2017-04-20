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
mod file_filter;

pub use self::config::*;
use self::file_filter::*;
use self::module_file::*;
use partial::*;
use session::*;
use front;
use middle;
use back;
use context::Context;
use ast::{JModule, JCrate};

static ABORT_MSG: &'static str = "stop due to compilation errors";

pub fn run() {
  let mut session = Session::new(Config::new());
  let context = front_mid_run(&mut session)
    .expect(ABORT_MSG);
  assert_eq!(context.session.has_errors(), false);
  run_back(context);
}

pub fn front_mid_run<'a>(session: &'a mut Session) -> Partial<Context<'a>> {
  run_front(session)
    .and_then(move |jcrate| run_middle(Context::new(session, jcrate)))
}

fn run_front(session: &mut Session) -> Partial<JCrate> {
  let file_filter = FileFilter::new(session.config());
  let mut jcrate = JCrate::new();
  for file in file_filter {
    run_front_module(session, file)
      .map(|module| jcrate.modules.push(module))
      .expect(ABORT_MSG);
  }
  Partial::Value(jcrate)
}

fn run_front_module(session: &mut Session, file: ModuleFile) -> Partial<JModule> {
  Partial::Value(session.load_file(file.input_path()))
    .and_then(front::parse_bonsai)
    .map(|ast| {
      for diagnostic in ast.expected_diagnostics.clone() {
        session.push_expected_diagnostic(diagnostic);
      }
      ast })
    .map(|ast| JModule::new(file, ast))
}

fn run_middle<'a>(context: Context<'a>) -> Partial<Context<'a>> {
  middle::analyse_bonsai(context)
}

fn run_back(mut context: Context) {
  for module in context.ast.modules.clone() {
    if !module.file.is_lib() {
      context.init_module(module.clone());
      let file = module.file.clone();
      back::generate_module(&context, module)
        .map(|output| file.write_output(output))
        .expect(ABORT_MSG);
    }
  }
}