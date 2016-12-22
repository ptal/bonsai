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

use driver::Config;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::{File, OpenOptions, DirBuilder};

#[derive(Clone)]
pub struct ModuleFile
{
  input_path: PathBuf,
  output_path: PathBuf,
  module_name: String
}

impl ModuleFile
{
  /// `project_path` is the path of the file relative to the root of the current project.
  pub fn new(config: &Config, project_path: PathBuf) -> Option<Self> {
    if project_path.extension().unwrap() == "java" {
      let p = project_path.clone();
      let bonsai_file = Path::new(p.file_stem().unwrap());
      if let Some(bonsai_ext) = bonsai_file.extension() {
        if bonsai_ext == "bonsai" {
          let mod_name = String::from(bonsai_file.file_stem().unwrap().to_str().unwrap());
          let mod_file = ModuleFile {
            input_path: Self::build_input_path(config, project_path.clone()),
            output_path: Self::build_output_path(config, project_path, mod_name.clone()),
            module_name: mod_name
          };
          return Some(mod_file);
        }
      }
    }
    None
  }

  fn build_input_path(config: &Config, project_path: PathBuf) -> PathBuf {
    config.input.join(&project_path)
  }

  fn build_output_path(config: &Config, mut project_path: PathBuf, mod_name: String) -> PathBuf {
    project_path.pop();
    let project_path = project_path
      .join(mod_name)
      .with_extension("java");
    config.output.join(&project_path)
  }

  pub fn mod_name(&self) -> String {
    self.module_name.clone()
  }

  pub fn input_path(&self) -> String {
    format!("{}", self.input_path.display())
  }

  pub fn input_as_string(&self) -> String {
    let mut file = File::open(self.input_path.clone())
      .expect(&format!("Input file ({})", self.input_path.to_str().unwrap_or("<invalid UTF8>")));
    let mut res = String::new();
    file.read_to_string(&mut res).unwrap();
    res
  }

  pub fn write_output(&self, output: String) {
    self.build_output_directory();
    let mut file = OpenOptions::new()
     .write(true)
     .truncate(true)
     .create(true)
     .open(self.output_path.clone())
     .expect(&format!("Output file ({})", self.output_path.to_str().unwrap_or("<invalid UTF8>")));
    file.write_fmt(format_args!("{}", output)).unwrap();
  }

  fn build_output_directory(&self) {
    if let Some(dir_path) = self.output_path.parent() {
      DirBuilder::new()
      .recursive(true)
      .create(dir_path)
      .expect("Recursive creation of directory for the output file.");
    }
  }
}
