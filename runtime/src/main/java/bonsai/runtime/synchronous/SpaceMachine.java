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

package bonsai.runtime.synchronous;

import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class SpaceMachine
{
  private Program body;
  private Environment env;

  public SpaceMachine(Program body, int numLayers) {
    this.body = body;
    this.env = new Environment(numLayers);
  }

  // Returns `true` if the program is paused (through a `stop` or `pause up` statement).
  // If the program is terminated, it returns `false`.
  public boolean execute() {
    CompletionCode code = executeLayer();
    return code != CompletionCode.TERMINATE;
  }

  CompletionCode executeLayer() {
    env.incTargetLayer();
    CompletionCode status = CompletionCode.PAUSE;
    while (status == CompletionCode.PAUSE) {
      body.prepareSubInstant(env, Environment.OUTERMOST_LAYER);
      // We execute as much as we can of the current instant.
      status = executeInstant();
      // If we are blocked but a sub-layer can be activated, we proceed.
      if (status == CompletionCode.PAUSE_DOWN) {
        CompletionCode subStatus = executeLayer();
        if (subStatus.isInternal()) {
          throw new RuntimeException("BUG: a layer cannot complete its execution on an internal completion code.");
        }
        // We execute the remaining of the current instant (in case the sub-layer wrote on variables of its parent's layer).
        status = executeInstant();
        if (status.isInternal()) {
          throw new RuntimeException("BUG: the sub-layer has been activated once, but the current instant is still blocked.");
        }
      }
    }
    env.decTargetLayer();
    return status;
  }

  CompletionCode executeInstant() {
    CompletionCode status = CompletionCode.WAIT;
    while (status == CompletionCode.WAIT) {
      status = body.executeSub(env, Environment.OUTERMOST_LAYER);
      Layer layer = env.targetLayer();
      if (status.isInternal() && !layer.processWasScheduled()) {
        boolean wasUnblocked = layer.unblock(body);
        if(!wasUnblocked) {
          if (status != CompletionCode.PAUSE_DOWN) {
            throw new RuntimeException("BUG: the current layer is blocked (every process waits for an event) and no sub-universe can be executed.");
          }
          break;
        }
      }
    }
    return status;
  }
}
