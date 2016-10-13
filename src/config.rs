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

use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use clap::App;

pub struct Config
{
  pub input: PathBuf,
  pub output: PathBuf
}

impl Config
{
  pub fn new() -> Config {
    let matches = App::new("bonsai")
      .version("0.1.0")
      .author("Pierre Talbot <ptalbot@hyc.io>")
      .about("Compiler of the Bonsai programming language.")
      .args_from_usage(
        "-o, --output=[filename] 'Write output to [filename]'
        <input>                  'Bonsai file to compile'")
      .get_matches();

    let input = PathBuf::from(matches.value_of("input").unwrap());
    let output = matches.value_of("output")
      .map(PathBuf::from)
      .unwrap_or(Config::default_output(&input));
    Config {
      input: input,
      output: output
    }
  }

  fn default_output(input: &PathBuf) -> PathBuf {
    input.with_extension("java")
  }

  pub fn input_as_string(&self) -> String {
    let mut file = File::open(self.input.clone()).unwrap();
    let mut res = String::new();
    file.read_to_string(&mut res).unwrap();
    res
  }

  pub fn write_output(&self, output: String) {
    let mut file = OpenOptions::new()
     .write(true)
     .truncate(true)
     .open(self.output.clone())
     .unwrap();
    file.write_fmt(format_args!("{}", output)).unwrap();
  }
}
