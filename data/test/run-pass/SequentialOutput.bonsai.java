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

#[run(new SequentialOutput().test(), "123")]

package test;

import bonsai.runtime.core.*;
import bonsai.runtime.choco.*;
import bonsai.runtime.sugarcubes.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class SequentialOutput
{
  proc test() {
    System.out.print("1");
  }
}
