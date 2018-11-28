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
import bonsai.runtime.synchronous.exceptions.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.search.*;

public class SpaceMachine
{
  private Statement program;
  private Environment env;
  private boolean debug;

  public SpaceMachine(Statement program, boolean debug) {
    this.env = new Environment(program.countLayers()+1);
    this.program = program;
    this.program.prepare();
    this.debug = debug;
  }

  // Returns `true` if the program is paused (through a `stop` or `pause up` statement).
  // If the program is terminated, it returns `false`.
  public boolean execute() {
    StmtResult res = executeLayer();
    // This part is useful for the automatic tests (because mvn exec does not flush automatically).
    System.out.flush();
    return res.k != CompletionCode.TERMINATE;
  }

  StmtResult executeLayer() {
    env.incTargetLayer();
    Layer layer = env.targetLayer();
    int targetIdx = env.targetIdx();
    StmtResult res = new StmtResult(CompletionCode.PAUSE);
    while (res.k == CompletionCode.PAUSE) {
      popQueues(targetIdx);
      program.canInstant(targetIdx, layer);
      // We execute as much as we can of the current instant.
      res = executeInstant(targetIdx, layer);
      // If we are blocked but a sub-layer can be activated, we proceed.
      if (res.k == CompletionCode.PAUSE_DOWN) {
        StmtResult subRes = executeLayer();
        if (subRes.k.isInternal()) {
          throw new CausalException("A layer cannot complete its execution on an internal completion code.");
        }
        // We execute the remaining of the current instant (in case the sub-layer wrote on variables of its parent's layer).
        res = executeInstant(targetIdx, layer);
        if (res.k.isInternal()) {
          throw new CausalException("The sub-layer has been activated once, but the current instant is still blocked.");
        }
      }
    }
    env.decTargetLayer();
    return res;
  }

  // We pop a future from all the active queues.
  // We merge all the future and
  //    1. We restore the set of variables world line.
  //    2. We execute the program "p1 || p2 || ... || pn" where "pi" represents one future.
  //       They can communicate on single space variables.
  // Since the captured space contains the same pointer to `Variable` than the current layer, the values are automatically updated in the layer.
  void popQueues(int layersRemaining) {
    HashSet<String> queues = program.activeQueues(layersRemaining);
    if (!queues.isEmpty()) {
      List<Future> futures = env.pop(queues);
      Future future = Future.merge(futures);
      future.space.restore();
      Statement mainProgram = program;
      program = future.body;
      Layer layer = new Layer(future.space);
      program.canInstant(0, layer);
      StmtResult res = executeInstant(0, layer);
      program = mainProgram;
      if (res.k != CompletionCode.TERMINATE) {
        throw new RuntimeException("A space statement did not terminate. (code: " + res.k + ")");
      }
      else if(!res.branchesPerQueue.isEmpty()) {
        throw new RuntimeException("A space statement contains nested space statement.");
      }
    }
  }

  StmtResult executeInstant(int layersRemaining, Layer layer) {
    StmtResult res = new StmtResult(CompletionCode.WAIT);
    while (res.k.isInternal()) {
      res = program.execute(layersRemaining, layer);
      if (res.k.isInternal() && !layer.processWasScheduled()) {
        boolean wasUnblocked = layer.unblock(program);
        if(!wasUnblocked) {
          if (res.k != CompletionCode.PAUSE_DOWN) {
            throw new CausalException("The current layer is blocked (every process waits for an event) and no sub-universe can be executed.");
          }
          break;
        }
      }
    }
    return res;
  }
}
