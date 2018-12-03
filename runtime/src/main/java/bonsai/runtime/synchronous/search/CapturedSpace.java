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

package bonsai.runtime.synchronous.search;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.variables.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.statements.*;
import bonsai.runtime.synchronous.interfaces.*;

// `CapturedSpace` is the variable's environment that is captured in `space` statements.
// Its purpose is to keep alive (at least one reference) variables that went out of scopes, and `single_time` variables because they are reinitialized at each instant.
// `CapturedSpace` is shared among all the children nodes created during an instant.
// The modifications performed on `single_space` or `world_line` variables through this class are automatically forwarded to the current space.
// The reason is that the variables in `memory` are the same than the ones of the current layer.
// If this pointer does exist anymore in the current layer, it means that the variable went out of scope, thus we only use its value in the current branch, and the changes are not forwarded.
public class CapturedSpace extends Space
{
  // It contains the label of the `world_line` variables (see `Restorable`).
  private HashMap<String, Object> labels;

  public CapturedSpace() {
    super();
    labels = new HashMap();
  }

  public CapturedSpace(HashMap<String, Variable> memory) {
    super(memory);
    labels = new HashMap();
  }

  public void registerWL(Variable var, boolean exitScope) {
    String uid = var.uid();
    boolean inMemory = memory.get(uid) != null;
    boolean inLabels = labels.get(uid) != null;
    // If `uid` is in `memory` but not in `labels` it means it is captured by a space statement.
    // The variables exit its scope and is not captured in a space statement, so we do not need to save it.
    if (!inMemory && !inLabels && exitScope) {
      return;
    }
    else {
      if (!inMemory) {
        memory.put(uid, var);
      }
      if (!inLabels) {
        Restorable r = Cast.toRestorable(uid, var.value());
        labels.put(uid, r.label());
      }
    }
  }

  public void restore() {
    for (Map.Entry<String, Object> label : labels.entrySet()) {
      Variable v = lookUpVar(label.getKey());
      Cast.toRestorable(label.getKey(), v.value()).restore(label.getValue());
    }
  }

  public void merge(CapturedSpace space) {
    super.merge(space);
    for (Map.Entry<String, Object> label : space.labels.entrySet()) {
      labels.computeIfAbsent(label.getKey(), k -> label.getValue());
    }
  }
}
