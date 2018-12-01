// Copyright 2018 Pierre Talbot

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
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.exceptions.*;
import bonsai.runtime.synchronous.env.*;

public class Universe extends QFUniverse
{
  private final String queueName;

  public Universe(String queueName, Statement body) {
    super(body);
    this.queueName = queueName;
  }

  // We terminate the statement if the queue is empty (but in the first instant).
  public StmtResult execute(int layersRemaining, Layer layer){
    if(layersRemaining == 1) {
      layer.enterQueue(queueName);
    }
    StmtResult res = super.execute(layersRemaining, layer);
    if(layersRemaining == 1) {
      layer.exitQueue();
    }
    // If the current universe does not wait to execute the body in the current instant (k != PAUSE_DOWN), then we terminate the statement if the queue is empty.
    // We do not check this condition during the first instant because "k == PAUSE_DOWN".
    if(layersRemaining == 0 && k != CompletionCode.PAUSE_DOWN) {
      Queueing queue = layer.getQueue(queueName);
      if (queue.isEmpty()) {
        k = CompletionCode.TERMINATE;
        res = new StmtResult(k);
      }
    }
    return res;
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    if (layersRemaining == 1) {
      HashSet<String> queues = new HashSet();
      queues.add(queueName);
      return queues;
    }
    else {
      return body.activeQueues(layersRemaining - 1);
    }
  }
}
