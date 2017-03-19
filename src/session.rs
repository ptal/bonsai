// Copyright 2017 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use driver::config::*;
use syntex_syntax::codemap::{FileMap, CodeMap};
use std::path::Path;
use std::rc::Rc;

pub struct Session {
  pub config: Config,
  pub codemap: CodeMap,
}

impl Session
{
  pub fn new(config: Config) -> Self {
    Session {
      config: config,
      codemap: CodeMap::new(),
    }
  }

  pub fn config<'a>(&'a self) -> &'a Config {
    &self.config
  }

  pub fn load_file(&mut self, path: &Path) -> Rc<FileMap> {
    self.codemap.load_file(path).unwrap()
  }
}
