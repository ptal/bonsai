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

package bonsai.chococubes.sugarcubes;

import java.util.*;
import bonsai.chococubes.core.*;
import inria.meije.rc.sugarcubes.implementation.*;
import inria.meije.rc.sugarcubes.*;

public class SpaceMachine extends StdMachine
{
  private boolean debug;

  public static final InternalIdentifiers INTERNAL_STRING_IDENTIFIERS =
    new InternalStringIdentifiers();

  public static SpaceMachine create(Program body) {
    Program p = new LinkObject(null,
      SC.shell(new StringID(".."), body),
      SC.NO_ACTION, SC.NO_ACTION, SC.NO_ACTION);
    SpaceEnvironment env = new SpaceEnvironment(
      ClockRegistry.noMultiClockMode(), INTERNAL_STRING_IDENTIFIERS, p);
    return new SpaceMachine(env);
  }

  public static SpaceMachine createDebug(Program body) {
    SpaceMachine machine = SpaceMachine.create(body);
    machine.debug = true;
    return machine;
  }

  public SpaceMachine(SpaceEnvironment env) {
    super(env);
    debug = false;
  }

  // Returns `true` if it stops because no more nodes are on the queue, otherwise `false` if the program terminated without consuming all nodes.
  public boolean execute() {
    SpaceEnvironment env = (SpaceEnvironment) clock0;
    if (debug) {
      System.out.println("[Start of execution]");
    }
    int numReactions;
    for (numReactions = 1; true; numReactions++) {
      if (debug) {
        System.out.println("[Reaction " + numReactions + "] Starting. Size of the reaction queue: " + env.queueSize());
      }
      if (react() || env.isEmpty()) {
        break;
      }
    }
    if (debug) {
      System.out.println("[End of execution] After " + numReactions + " reactions due to " +
        ((env.isEmpty()) ? "empty reaction queue.":"program termination."));
    }
    return env.isEmpty();
  }
}