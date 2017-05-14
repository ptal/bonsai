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

#[error(E0024, 26, 25)]
#[error(E0024, 27, 25)]

package test;

public class ModuleParameterMismatch
{
  public single_space T a;
  public single_space T b;

  proc test() {
    module Module2 ko1 = new Module2(a, b);
    module Module2 ko2 = new Module2();
  }
}
