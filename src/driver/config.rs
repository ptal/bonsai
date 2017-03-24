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

/// This module processes the command-line arguments and performs basic checks.

use std::path::PathBuf;
use clap::{App, Error, ErrorKind};

pub struct Config
{
  pub input: PathBuf,
  pub output: PathBuf,
  pub libs: Vec<PathBuf>,
  pub main_method: Option<String>,
  pub debug: bool,
  pub testing_mode: bool
}

static EXEC_NAME: &'static str = "bonsai";

impl Config
{
  pub fn new() -> Config {
    let matches = App::new(EXEC_NAME)
      .version("0.1.0")
      .author("Pierre Talbot <ptalbot@hyc.io>")
      .about("Compiler of the Bonsai programming language.")
      .args_from_usage(
        "-o, --output=[directory] 'Write compiled bonsai files to [directory]. The directory structure of the input project is preserved.'
        --main=[classname]        'Generate a method main in the class [classname] for immediate testing.'
        --debug                   'Generate code with debug facility.'
        --lib=[directory]...      'Paths to bonsai libraries used inside this project. The code is not compiled to Java so you still have to import the .jar of these libraries in your project.'
        <input>                   'Root of the bonsai project to compile. All files terminating with the `.bonsai` extension are compiled.'")
      .get_matches();

    let libs: Vec<_> = matches.values_of("lib")
      .map(|libs| libs.map(PathBuf::from).collect())
      .unwrap_or(vec![]);

    let input = PathBuf::from(matches.value_of("input").unwrap());
    let output = matches.value_of("output")
      .map(|s| s.trim())
      .map(PathBuf::from)
      .unwrap_or(Config::default_output(&input));
    let config = Config {
      input: input,
      output: output,
      libs: libs,
      main_method: matches.value_of("main").map(String::from),
      debug: matches.is_present("debug"),
      testing_mode: false
    };
    config.validate();
    config
  }

  #[allow(dead_code)]
  pub fn testing_mode(file_to_test: PathBuf, libs: Vec<PathBuf>) -> Config {
    Config {
      input: file_to_test.clone(),
      output: file_to_test,
      libs: libs,
      main_method: None,
      debug: true,
      testing_mode: true
    }
  }

  fn default_output(input: &PathBuf) -> PathBuf {
    input.clone()
  }

  fn validate(&self) {
    Config::check_is_dir(&self.input, "input", true);
    Config::check_is_dir(&self.output, "output", false);
    for lib in &self.libs {
      Config::check_is_dir(lib, "library", true);
    }
  }

  fn check_is_dir(path: &PathBuf, name: &str, must_exist: bool) {
    // Don't generate error if the path is a directory OR if the path does not exist and is not forced to exist.
    if !path.is_dir() {
      if !must_exist && !path.exists() { return; }
      Error::with_description(&format!(
        "The {} path `{}` is not a directory. See `{} --help` for more information.",
          name, path.display(), EXEC_NAME),
        ErrorKind::ValueValidation)
      .exit();
    }
  }
}
