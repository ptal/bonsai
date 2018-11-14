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
  private boolean debug;

  public SpaceMachine(Program body, int numLayers, boolean debug) {
    if (numLayers < 0) {
      throw new RuntimeException("SpaceMachine: The number of layers cannot be negative.");
    }
    this.body = body;
    this.body.prepare();
    this.env = new Environment(numLayers+1);
    this.debug = debug;
  }

  // Returns `true` if the program is paused (through a `stop` or `pause up` statement).
  // If the program is terminated, it returns `false`.
  public boolean execute() {
    CompletionCode code = executeLayer();
    return code != CompletionCode.TERMINATE;
  }

  CompletionCode executeLayer() {
    env.incTargetLayer();
    Layer layer = env.targetLayer();
    int targetIdx = env.targetIdx();
    CompletionCode k = CompletionCode.PAUSE;
    while (k == CompletionCode.PAUSE) {
      body.canInstant(targetIdx, layer);
      // We execute as much as we can of the current instant.
      k = executeInstant(targetIdx, layer);
      // If we are blocked but a sub-layer can be activated, we proceed.
      if (k == CompletionCode.PAUSE_DOWN) {
        CompletionCode subK = executeLayer();
        if (subK.isInternal()) {
          throw new CausalException("A layer cannot complete its execution on an internal completion code.");
        }
        // We execute the remaining of the current instant (in case the sub-layer wrote on variables of its parent's layer).
        k = executeInstant(targetIdx, layer);
        if (k.isInternal()) {
          throw new CausalException("The sub-layer has been activated once, but the current instant is still blocked.");
        }
      }
    }
    env.decTargetLayer();
    return k;
  }

  CompletionCode executeInstant(int layersRemaining, Layer layer) {
    CompletionCode k = CompletionCode.WAIT;
    while (k == CompletionCode.WAIT) {
      k = body.execute(layersRemaining, layer);
      if (k.isInternal() && !layer.processWasScheduled()) {
        boolean wasUnblocked = layer.unblock(body);
        if(!wasUnblocked) {
          if (k != CompletionCode.PAUSE_DOWN) {
            throw new CausalException("The current layer is blocked (every process waits for an event) and no sub-universe can be executed.");
          }
          break;
        }
      }
    }
    return k;
  }
}
