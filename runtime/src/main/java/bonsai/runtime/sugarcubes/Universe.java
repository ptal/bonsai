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

package bonsai.runtime.sugarcubes;

import java.util.function.*;
import bonsai.runtime.core.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class Universe extends UnaryInstruction
{
  public SpaceMachine machine;
  public boolean debug;

  public Universe(boolean debug, Program body)
  {
    super(body);
    this.debug = debug;
    if (debug) {
      machine = SpaceMachine.createDebug(body);
    }
    else {
      machine = SpaceMachine.create(body);
    }
  }

  public String actualToString() {
    return "universe " + body + " end";
  }

  public Universe copy() {
    return new Universe(debug, body);
  }

  public Universe prepareFor(Environment env) {
    Universe copy = new Universe(debug, body.prepareFor(env));
    copy.body.setParent(copy);
    return copy;
  }

  public byte activate(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    MachineStatus status = machine.execute();
    switch (status) {
      case PausedUp: return STOP;
      case Stopped:  env.stopped = true; return STOP;
      case Terminated:
      default:
        return TERM;
    }
  }
}
