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

#![feature(plugin, box_syntax, rustc_private)]
#![plugin(oak)]

extern crate libbonsai;
extern crate syntex_syntax;
extern crate syntex_errors;
extern crate syntex_pos;
extern crate term;
extern crate partial;

mod test;

use test::*;

use std::path::{PathBuf, Path};

fn test_data_dir(filter: bool) {
  let data_path = Path::new("data/");
  if !data_path.is_dir() {
    panic!(format!("`{}` is not a valid data directory.", data_path.display()));
  }
  let mut test_path = PathBuf::new();
  test_path.push(data_path);
  test_path.push(Path::new("test"));
  let test_lib = PathBuf::from("data/test/lib");
  if !test_lib.is_dir() {
    panic!(format!("`{}` must be a directory (the bonsai library used in the test files).", test_lib.display()));
  }
  let mut engine = Engine::new(test_path, test_lib, filter);
  engine.run();
}

#[test]
fn test_data_directory()
{
  test_data_dir(false);
}

// #[test]
// fn debug_run() {
//   test_data_dir(true);
// }
