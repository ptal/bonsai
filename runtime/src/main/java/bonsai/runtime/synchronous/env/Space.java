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

package bonsai.runtime.synchronous.env;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.variables.*;
import bonsai.runtime.synchronous.interfaces.*;

// `Space` is the variable's environment.
// The life cycle of a variable is as follows: `register` (called during the `canAnalysis`), `enterScope` and finally `exitScope`.
public class Space
{
  // This is the memory of all variables, regardless of their spacetime.
  // All variables in `memory` are registered or in scope.
  protected HashMap<String, Variable> memory;

  public Space() {
    memory = new HashMap();
  }

  protected Space(HashMap<String, Variable> memory) {
    this.memory = memory;
  }

  public Variable lookUpVar(String uid) {
    Variable v = memory.get(uid);
    checkVarNull(v, uid);
    return v;
  }

  private void checkVarNull(Variable v, String uid) {
    if (v == null) {
      throw new RuntimeException("The variable `" + uid
        + "` is not registered in `Space.memory`.");
    }
  }

  private void checkNullUID(String uid, String from) {
    if (uid == null) {
      throw new RuntimeException(from + ": null `uid` parameter.");
    }
  }

  public void enterScope(String uid, Object defaultValue, Consumer<Object> refUpdater) {
    // System.out.println("enterScope" + uid);
    checkNullUID(uid, "Space.enterScope");
    Variable var = memory.get(uid);
    if (var == null) {
      throw new RuntimeException(
        "Space.enterScope: The variable with UID `" + uid + "` is in scope, but " +
        "it is not in `memory`.");
    }
    var.enterScope(defaultValue, refUpdater);
  }

  public void exitScope(String uid) {
    // System.out.println("exitScope" + uid);
    checkNullUID(uid, "Space.exitScope");
    Variable v = memory.get(uid);
    if (v == null) {
      throw new RuntimeException(
        "Space.exitScope: The variable with UID `" + uid + "` is in scope, but " +
        "it is not in `memory`.");
    }
    else {
      v.exitScope();
      // If all the refs to this variable exit.
      if(!v.isInScope()) {
        memory.remove(uid);
      }
    }
  }

  public void register(String uid, boolean overwrite) {
    // System.out.println("register" + uid);
    checkNullUID(uid, "Space.register");
    if (overwrite) {
      memory.compute(uid, (k,v) -> new Variable(k));
    }
    else {
      memory.computeIfAbsent(uid, k -> new Variable(k));
    }
  }

  // It merges the variables between two spaces.
  // Precondition: Variables present in two spaces must have the same value.
  public void merge(Space space) {
    for (Map.Entry<String, Variable> var : space.memory.entrySet()) {
      memory.computeIfAbsent(var.getKey(), k -> var.getValue());
    }
  }

  public HashMap<String, Variable> project(ArrayList<String> varsUIDs) {
    HashMap<String, Variable> projectedMemory = new HashMap();
    for(String uid : varsUIDs) {
      projectedMemory.put(uid, memory.get(uid));
    }
    return projectedMemory;
  }

  // Unblocking processes on condition follows three steps:
  //  1. Call `canWriteOn(x)` where `x` is a variable with a write/readwrite counter > 0.
  //  2. Record all conditions that are unblocked on this variable with `layer.subscribeUnblocked`.
  //  3a. If `canWriteOn(x) == false`, then we cannot write on this variable anymore:
  //      We commit to the result recorded at step (2) for conditions on this variables.
  //  3b. Otherwise we unsubscribe the unblocked condition.
  public boolean unblock(Statement body, int layersRemaining, Layer layer) {
    boolean unblocked = false;
    for (Variable var : memory.values()) {
      if (!var.isReadable()) {
        // System.out.println("unblock: " + var.uid());
        if(!body.canWriteOn(layersRemaining, layer, var.uid(), true).canWrite) {
          unblocked = true;
          layer.scheduleUnblocked(var.uid());
        }
        else {
          layer.unsubscribeUnblocked(var.uid());
        }
      }
    }
    return unblocked;
  }
}
