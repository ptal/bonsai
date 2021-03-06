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

/// `FileFilter` collects the `.bonsai` files of the input directory.

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
pub struct FileFilter
{
  mod_to_files: HashMap<String, ModuleFile>
}

impl FileFilter
{
  pub fn new(config: &Config) -> Self {
    let mut package = FileFilter {
      mod_to_files: HashMap::new()
    };
    let err_msg = "Failed to collect bonsai files.";
    for lib in &config.libs {
      package.collect_bonsai_files(config, true, lib.clone())
        .expect(&format!("{:?}: {}", lib, err_msg));
    }
    // When testing, the input in `config` is a file, not a directory.
    if config.testing_mode {
      let file =  ModuleFile::new(config, config.input.clone(), false)
      .expect(&format!("Testing file {:?} is not a `.bonsai.java` file.", config.input));
      package.add_mod_file(file);
    }
    else {
      package.collect_bonsai_files(config, false, config.input.clone())
        .expect(&format!("{:?}: {}", config.input, err_msg));
    }
    package
  }

  fn collect_bonsai_files(&mut self, config: &Config, lib: bool,
    current_dir: PathBuf) -> io::Result<()>
  {
    for entry in read_dir(current_dir)? {
      let entry = entry?.path();
      if entry.is_dir() {
        self.collect_bonsai_files(config, lib, entry)?;
      }
      else {
        if let Some(mod_file) = ModuleFile::new(config, entry, lib) {
          self.add_mod_file(mod_file)
        }
      }
    }
    Ok(())
  }

  fn add_mod_file(&mut self, mod_file: ModuleFile) {
    let mod_name = mod_file.mod_name();
    if self.mod_to_files.contains_key(&mod_name) {
      self.conflicting_module_error(mod_name, mod_file);
    }
    else {
      self.mod_to_files.insert(mod_name, mod_file);
    }
  }

  fn conflicting_module_error(&self, conflict_mod: String, mod_file: ModuleFile) {
    let existing_file = self.mod_to_files[&conflict_mod].clone();
    Error::with_description(
      &format!("Module {} already imported. Conflicting modules:\n\
                 {} ({})\n\
                 {} ({})\n\
               Explanation: Modules must have a distinct name because bonsai does not have a namespace mechanism.\n\
               Solution: Rename one of these modules.",
        conflict_mod,
        conflict_mod, mod_file.input_path_str(),
        conflict_mod, existing_file.input_path_str()),
      ErrorKind::ValueValidation
    ).exit();
  }
}

impl IntoIterator for FileFilter
{
  type Item = ModuleFile;
  type IntoIter = vec::IntoIter<ModuleFile>;

  fn into_iter(self) -> vec::IntoIter<ModuleFile> {
    let values: Vec<_> = self.mod_to_files.into_iter()
      .map(|(_,v)| v).collect();
    values.into_iter()
  }
}

