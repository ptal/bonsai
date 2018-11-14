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
  private int targetIdx;

  public Environment(int numLayers)
  {
    layers = new ArrayList(numLayers);
    for(int i=0; i < numLayers; i++) {
      layers.add(new Layer());
    }
    targetIdx = OUTERMOST_LAYER;
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
}
