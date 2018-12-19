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
  private boolean firstInstant;

  public Universe(String queueName, Statement body) {
    super(body);
    this.queueName = queueName;
  }

  public void prepare() {
    super.prepare();
    firstInstant = true;
  }

  public Universe copy() {
    // throw new CannotCopyException("Universe");
    return new Universe(queueName, body.copy());
  }

  public StmtResult execute(int layersRemaining, Layer layer) {
    if(layersRemaining == 1) {
      layer.enterQueue(queueName);
    }
    StmtResult res = super.execute(layersRemaining, layer);
    if(layersRemaining == 1) {
      layer.exitQueue();
    }
    return res;
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    checkNonTerminatedEOI("universe with q in p end", res.k);
    firstInstant = false;
    if (layersRemaining > 1) {
      return body.endOfInstant(layersRemaining - 1, layer);
    }
    else if(layersRemaining == 1) {
      Queueing queue = layer.parent().getQueue(queueName);
      if (queue.isEmpty()) {
        bodyRes = new StmtResult(CompletionCode.TERMINATE);
        res = new StmtResult(CompletionCode.TERMINATE);
      }
      return bodyRes.k;
    }
    return res.k;
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    if(layersRemaining <= 1) {
      if (layersRemaining == 1 && res.k != CompletionCode.TERMINATE && !firstInstant) {
        HashSet<String> queues = new HashSet();
        queues.add(queueName);
        return queues;
      }
      else {
        return new HashSet();
      }
    }
    else {
      return body.activeQueues(layersRemaining - 1);
    }
  }
}
