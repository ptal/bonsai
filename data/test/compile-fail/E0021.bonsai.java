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

#[error(E0021, 29, 9)]
#[error(E0021, 30, 9)]

package test;

import test.Module;
import test.Module2;

public class ModuleRefFieldInitializer
{
  public single_space T a;
  public module Module ok1;
  public module Module ok2 = new Module();
  public module Module2 ok3;
  public module Module2 ko1 = new Module2(a);
  public module Module2 ko2 = Module2.create(a);

  public proc test() = nothing
}
