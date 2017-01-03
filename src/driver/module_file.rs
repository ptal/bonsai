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

#[derive(Clone, Debug)]
pub struct ModuleFile
{
  input_path: PathBuf,
  output_path: Option<PathBuf>,
  module_name: String
}

impl ModuleFile
{
  pub fn new(config: &Config, file_path: PathBuf, lib: bool) -> Option<Self> {
    if file_path.extension().unwrap() == "java" {
      let p = file_path.clone();
      let bonsai_file = Path::new(p.file_stem().unwrap());
      if let Some(bonsai_ext) = bonsai_file.extension() {
        if bonsai_ext == "bonsai" {
          let mod_name = String::from(bonsai_file.file_stem().unwrap().to_str().unwrap());
          let mod_file =
            match lib {
              false => Self::core_file(config, file_path, mod_name),
              true => Self::lib_file(file_path, mod_name),
            };
          return Some(mod_file);
        }
      }
    }
    None
  }

  pub fn is_lib(&self) -> bool {
    self.output_path.is_none()
  }

  fn core_file(config: &Config, file_path: PathBuf, mod_name: String) -> Self {
    ModuleFile {
      input_path: file_path.clone(),
      output_path: Some(Self::build_output_path(config, file_path)),
      module_name: mod_name
    }
  }

  fn lib_file(file_path: PathBuf, mod_name: String) -> Self {
    ModuleFile {
      input_path: file_path.clone(),
      output_path: None,
      module_name: mod_name
    }
  }

  fn build_output_path(config: &Config, file_path: PathBuf) -> PathBuf {
    config.output.join(
      file_path.strip_prefix(&config.input)
      .expect("`file_path` should start with `config.input` when building output path of core files."))
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
    let output_path = self.output_path.clone().expect(
      "Try to compile a library file (this is a bug).");
    self.build_output_directory(output_path.clone());
    let mut file = OpenOptions::new()
     .write(true)
     .truncate(true)
     .create(true)
     .open(output_path.clone())
     .expect(&format!("Output file ({})", output_path.to_str().unwrap_or("<invalid UTF8>")));
    file.write_fmt(format_args!("{}", output)).unwrap();
  }

  fn build_output_directory(&self, output_path: PathBuf) {
    if let Some(dir_path) = output_path.parent() {
      DirBuilder::new()
      .recursive(true)
      .create(dir_path)
      .expect("Recursive creation of directory for the output file.");
    }
  }
}
