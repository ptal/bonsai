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

package bonsai.runtime.synchronous.env;

import java.util.*;
import java.util.function.*;
import java.util.stream.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.search.*;
import bonsai.runtime.synchronous.variables.*;
import bonsai.runtime.synchronous.interfaces.*;

public class Environment
{
  public static int OUTERMOST_LAYER = -1;
  private ArrayList<Layer> layers;
  private int targetIdx;

  public Environment(Layer first, int numLayers)
  {
    layers = new ArrayList(numLayers);
    layers.add(first);
    for(int i=1; i < numLayers; i++) {
      layers.add(new Layer());
    }
    for(int i=1; i <numLayers; i++) {
      layers.get(i).setParent(layers.get(i-1));
    }
    targetIdx = OUTERMOST_LAYER;
  }

  public Environment(int numLayers) {
    this(new Layer(), numLayers);
  }

  public void incTargetLayer() {
    targetIdx += 1;
    if (targetIdx >= layers.size()) {
      throw new RuntimeException("Environment.incTargetLayer: Enter a layer that is not registered in the environment.");
    }
  }
  public void decTargetLayer() {
    targetIdx -= 1;
  }
  public Layer targetLayer() {
    return layers.get(targetIdx);
  }
  public int targetIdx() {
    return targetIdx;
  }

  private Queueing getQueue(String name) {
    return layers.get(targetIdx - 1).getQueue(name);
  }

  public void push(HashMap<String, List<Future>> futuresPerQueue) {
    if (futuresPerQueue.size() > 0) {
      ensureNoTopLayer();
      for(Map.Entry<String, List<Future>> entry : futuresPerQueue.entrySet()) {
        Queueing queue = getQueue(entry.getKey());
        queue.push(entry.getValue());
      }
    }
  }

  public List<Future> pop(HashSet<String> queues) {
    ensureNoTopLayer();
    ArrayList<Future> futures = new ArrayList();
    for (String name: queues) {
      Queueing queue = getQueue(name);
      futures.add(toFuture(name, queue.pop()));
    }
    return futures;
  }

  private void ensureNoTopLayer() {
    if (targetIdx < 1) {
      throw new RuntimeException("[BUG] Environment.ensureNoTopLayer: Try to push a future onto a queue in the top-level universe.");
    }
  }

  static private void checkFutureObject(String var, Object o) {
    if (!(o instanceof Future)) {
      throw new RuntimeException(
        "`pop` on the queue `" + var + "` did not return a `Future` element. Object: " + o);
    }
  }

  static private Future toFuture(String var, Object o) {
    checkFutureObject(var, o);
    return (Future) o;
  }
}
