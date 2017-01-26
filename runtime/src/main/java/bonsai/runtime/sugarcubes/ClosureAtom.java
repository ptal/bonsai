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

package bonsai.runtime.sugarcubes;

import java.util.function.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class ClosureAtom extends Atom
{
  private Consumer<SpaceEnvironment> closure;

  public ClosureAtom(Consumer<SpaceEnvironment> closure) {
    this.closure = closure;
  }

  public String actualToString() {
    return "<closure atom>";
  }

  public ClosureAtom copy() {
    return new ClosureAtom(closure);
  }

  public ClosureAtom prepareFor(Environment env) {
    return copy();
  }

  public boolean action(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    closure.accept(env);
    return false;
  }
}
