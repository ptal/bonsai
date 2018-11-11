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
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.variables.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.statements.SpaceStmt;

public class Environment
{
  public static int OUTERMOST_LAYER = -1;
  private ArrayList<Layer> layers;
  private int targetLayer;

  public Environment(int numLayers)
  {
    layers = new ArrayList(numLayers);
    for(int i=0; i < numLayers; i++) {
      layers.add(new Layer());
    }
    targetLayer = OUTERMOST_LAYER;
  }

  public void incTargetLayer() {
    targetLayer += 1;
  }
  public void decTargetLayer() {
    targetLayer -= 1;
  }
  public Layer targetLayer() {
    return layers.get(targetLayer);
  }

  public CompletionCode traverseLayer(int currentLayer,
   Function<Layer, CompletionCode> execute,
   BiFunction<Environment, Integer, CompletionCode> executeSub) {
    currentLayer += 1;
    if (currentLayer == targetLayer) {
      return execute.apply(layers.get(currentLayer));
    }
    else if (currentLayer < targetLayer) {
      return executeSub.apply(this, currentLayer);
    }
    else {
      return CompletionCode.PAUSE_DOWN;
    }
  }

  public void traverseLayerPrepare(int currentLayer,
   Consumer<Layer> prepare,
   BiConsumer<Environment, Integer> prepareSub) {
    currentLayer += 1;
    if (currentLayer == targetLayer) {
      prepare.accept(layers.get(currentLayer));
    }
    else if(currentLayer < targetLayer) {
      prepareSub.accept(this, currentLayer);
    }
  }
}