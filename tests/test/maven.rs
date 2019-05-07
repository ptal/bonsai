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
use std::process::{Command, Stdio, Output};
use std::io;

pub struct Maven {
  sandbox: PathBuf,
  filter_debug: bool
}

impl Maven {
  pub fn new(root: PathBuf, filter_debug: bool) -> Self {
    Maven {
      sandbox: root.join("sandbox/"), filter_debug
    }
  }

  pub fn source_path(&self) -> PathBuf {
    let sandbox = self.sandbox.clone();
    sandbox.join("src/main/java/test/")
  }

  pub fn delete_source_files(&self) {
    let source_path = self.source_path();
    if source_path.exists() {
      let _ = fs::remove_dir_all(self.source_path());
      let _ = fs::remove_dir_all(self.sandbox.join("target/"));
    }
  }

  // mvn compile
  pub fn compile_sandbox(&self) -> io::Result<Output> {
    let child = Command::new("mvn")
      .arg("compile")
      .current_dir(self.sandbox.clone())
      .stdout(Stdio::piped())
      .spawn()?;
    child.wait_with_output()
  }

  // mvn -B -q exec:java -Dexec.mainClass="test.<class_name>"
  pub fn execute_sandbox(&self, main_class: String) -> io::Result<Output> {
    let main_class_arg = format!("-Dexec.mainClass=test.{}", main_class);
    let silent = if self.filter_debug { "-q" } else { "-q" };
    let child = Command::new("mvn")
      .args(&["-B", silent, "exec:java", &main_class_arg])
      .current_dir(self.sandbox.clone())
      .stdout(Stdio::piped())
      .spawn()?;

    child.wait_with_output()
  }
}
