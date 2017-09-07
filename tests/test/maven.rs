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

/// `Maven` is used to retrieve paths or generate files relevant to the Maven building system.

use std::path::{PathBuf};
use std::fs;

pub struct Maven {
  sandbox: PathBuf
}

impl Maven {
  pub fn new(root: PathBuf) -> Self {
    Maven {
      sandbox: root.join("sandbox/")
    }
  }

  pub fn source_path(&self) -> PathBuf {
    let sandbox = self.sandbox.clone();
    sandbox.join("src/main/java/test/")
  }

  pub fn delete_source_files(&self) {
    let source_path = self.source_path();
    if source_path.exists() {
      fs::remove_dir_all(self.source_path()).unwrap();
    }
  }
}
