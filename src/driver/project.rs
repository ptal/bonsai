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

/// `Project` collects the `.bonsai` files of the input directory.

use driver::Config;
use driver::module_file::ModuleFile;
use std::path::{PathBuf};
use std::fs::*;
use std::io;
use std::iter::IntoIterator;
use std::vec;
use std::collections::HashMap;
use clap::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Project
{
  mod_to_files: HashMap<String, ModuleFile>
}

impl Project
{
  pub fn new(config: &Config) -> Self {
    let mut package = Project {
      mod_to_files: HashMap::new()
    };
    package.find_bonsai_files(config, PathBuf::new()).unwrap();
    package
  }

  /// config.input ⁺ current_path = full path to the current directory.
  /// `current_path` is the current path relative to the root of the project.
  fn find_bonsai_files(&mut self, config: &Config, current_path: PathBuf) -> io::Result<()> {
    let current_dir = config.input.join(current_path.clone());
    for entry in read_dir(current_dir)? {
      let entry = entry?;
      let full_path = entry.path();
      let project_path = current_path.join(entry.file_name());
      if full_path.is_dir() {
        self.find_bonsai_files(config, project_path)?;
      }
      else if let Some(mod_file) = ModuleFile::new(config, project_path) {
        let mod_name = mod_file.mod_name();
        if self.mod_to_files.contains_key(&mod_name) {
          self.conflicting_module_error(mod_name, mod_file);
        }
        else {
          self.mod_to_files.insert(mod_name, mod_file);
        }
      }
    }
    Ok(())
  }

  fn conflicting_module_error(&self, conflict_mod: String, mod_file: ModuleFile) {
    let existing_file = self.mod_to_files[&conflict_mod].clone();
    Error::with_description(
      &format!("Module {} already imported. Conflicting modules:\n\
                 {} ({})\n\
                 {} ({})\n\
               Explanation: Modules must have a distinct name because bonsai does not have a namespace mechanism.\n\
               Resolution: Rename one of these modules.",
        conflict_mod,
        conflict_mod, mod_file.input_path(),
        conflict_mod, existing_file.input_path()),
      ErrorKind::ValueValidation
    ).exit();
  }
}

impl IntoIterator for Project
{
  type Item = ModuleFile;
  type IntoIter = vec::IntoIter<ModuleFile>;

  fn into_iter(self) -> vec::IntoIter<ModuleFile> {
    let values: Vec<_> = self.mod_to_files.into_iter()
      .map(|(_,v)| v).collect();
    values.into_iter()
  }
}

