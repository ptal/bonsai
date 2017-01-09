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

package bonsai.cp.core;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import bonsai.runtime.core.*;
import bonsai.runtime.choco.*;
import bonsai.runtime.sugarcubes.*;

public class Propagation implements Executable
{
  private channel world_line VarStore domains = bot;
  private channel world_line ConstraintStore constraints = bot;
  private channel single_time FlatLattice<Consistent> consistent = bot;

  public proc execute() {
    loop {
      consistent <- PropagatorEngine.propagate(domains, constraints);
      pause;
    }
  }
}
