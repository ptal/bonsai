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
use session::*;
use front;
use middle;
use back;
use context::Context;
use ast::{JModule, JCrate, TestAnnotation};

static ABORT_MSG: &'static str = "stop due to compilation errors";

pub fn run() {
  let session = Session::new(Config::new());
  front_mid_run(session)
    .and_next(run_back)
    .expect(ABORT_MSG);
}

pub fn front_mid_run<'a>(session: Session) -> Env<Context> {
  let env = run_front(session)
    .map(|jcrate| Context::new(jcrate))
    .ensure(ABORT_MSG);
  run_middle(env)
}

fn run_front(session: Session) -> Env<JCrate> {
  FileFilter::new(session.config())
    .into_iter()
    .fold(Env::value(session, JCrate::new()), run_front_module)
}

fn run_front_module(env: Env<JCrate>, file: ModuleFile) -> Env<JCrate> {
  env.and_then(|mut session, mut jcrate| {
    let content = session.load_file(file.input_path());
    let ast = front::parse_bonsai(content).expect(ABORT_MSG);
    for test in ast.tests.clone() {
      match test {
        TestAnnotation::Compiler(test) => session.push_compiler_test(test),
        TestAnnotation::Execution(test) => session.push_execution_test(test)
      }
    }
    jcrate.modules.push(JModule::new(file, ast));
    Env::value(session, jcrate)
  })
}

fn run_middle<'a>(env: Env<Context>) -> Env<Context> {
  middle::analyse_bonsai(env)
}

pub fn run_back(session: Session, context: Context) -> Env<Context> {
  assert_eq!(session.has_errors(), false);
  context.ast.modules.clone()
    .into_iter()
    .filter(|module| !module.file.is_lib())
    .fold(Env::value(session, context), |env, module| {
      let file = module.file.clone();
      back::compile_module(env, module)
        .map(|(context, output)| {
          file.write_output(output);
          context
        })
        .ensure(ABORT_MSG)
    })
}
