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
import bonsai.runtime.synchronous.variables.*;

public class Space
{
  // This is the memory of all variables, regardless of their spacetime.
  // Variables that are not in scope anymore, but captured in a `space` branch, are kept alive in `memory`.
  private ArrayList<Variable> memory;
  // The index of free cells in `memory` that can be reallocated.
  private ArrayList<Integer> freeCells;

  // The next three hash maps classify the variables depending on their spacetime.
  // All the variables currently in scope are in these HashMap.
  private HashMap<Integer, SingleTimeVar> varsST;
  private HashMap<Integer, StreamVar> varsSS;
  private HashMap<Integer, StreamVar> varsWL;

  // `scope` maps the variables names in scope to their UIDs.
  // We have an `ArrayList` of UIDs in case a name is used several times.
  // (it might be possible to remove ArrayList if we uniquely identify each syntactic variable).
  private HashMap<String, ArrayList<Integer>> scope;

  // private HashMap<Integer, ModuleVar> varsModule;
  // private HashMap<Integer, Consumer<Space>> refsUpdaters;

  public Space()
  {
    memory = new ArrayList();
    freeCells = new ArrayList();
    varsST = new HashMap();
    varsSS = new HashMap();
    varsWL = new HashMap();
    scope = new HashMap();
  }

  // Look up a variable: `memory[scope[name]]`.
  public Variable lookUpVar(String name) {
    ArrayList<Integer> uids = scope.get(name);
    checkUIDsNull(uids, name);
    Integer uid = uids.get(uids.size()-1);
    Variable v = memory.get(uid);
    checkVarNull(uid, v);
    return v;
  }

  public void checkUIDsNull(ArrayList<Integer> uids, String name) {
    if (uids == null) {
      throw new RuntimeException("The variable `" + name
        + "` is not registered in the environment `scope`.");
    }
    else if (uids.size() == 0) {
      throw new RuntimeException("The variable `" + name
        + "` is registered in `scope` but has an empty list of UIDs.");
    }
  }

  public void checkVarNull(Integer uid, Variable v) {
    if (v == null) {
      throw new RuntimeException("The variable `" + v.name() + " (uid: " + uid + ")"
        + "` is not registered in the environment `memory`.");
    }
  }

  private void allocateVar(Variable var) {
    if (freeCells.isEmpty()) {
      var.assignUID(memory.size());
      memory.add(var);
    }
    else {
      var.assignUID(freeCells.remove(freeCells.size()-1));
      memory.set(var.uid(), var);
    }
  }

  // Decrease the reference counter of `var`, and delete the variable from `memory` if it reached 0.
  private void deallocateVar(Variable var) {
    var.decreaseRefs();
    if (var.refs() == 0) {
      if (memory.size() - 1 == var.uid()) {
        memory.remove(var.uid());
      }
      else {
        memory.set(var.uid(), null);
        freeCells.add(var.uid());
      }
    }
  }

  private <T extends Variable> void enterScope(T var, HashMap<Integer, T> vars) {
    if (var == null) {
      throw new RuntimeException("Space.enterScope: null `var` parameter.");
    }
    allocateVar(var);
    scope
      .computeIfAbsent(var.name(), k -> new ArrayList<>())
      .add(var.uid());
    vars.put(var.uid(), var);
  }

  private <T extends Variable> void exitScope(String name, HashMap<Integer, T> vars) {
    if (name == null) {
      throw new RuntimeException("Space.exitScope: null `name` parameter.");
    }
    // Remove the scope entry `(name, uid)`.
    ArrayList<Integer> uids = scope.get(name);
    checkUIDsNull(uids, name);
    Integer uid = uids.remove(uids.size() - 1);
    // Remove the variable from the corresponding spacetime HashMap.
    Variable removed = vars.remove(uid);
    if (removed == null) {
      throw new RuntimeException(
        "Space.exitScope: The variable with name `" + name + "` is in scope, but " +
        "it is not in the corresponding variable HashMap.");
    }
    deallocateVar(removed);
  }

  public void enterScopeST(SingleTimeVar var) { enterScope(var, this.varsST); }
  public void enterScopeSS(StreamVar var) { enterScope(var, this.varsSS); }
  public void enterScopeWL(StreamVar var) { enterScope(var, this.varsWL); }
  // public void enterScopeModule(ModuleVar var) { enterScope(var, this.varsModule); }
  public void exitScopeST(String name) { exitScope(name, this.varsST); }
  public void exitScopeSS(String name) { exitScope(name, this.varsSS); }
  public void exitScopeWL(String name) { exitScope(name, this.varsWL); }
  // public void exitScopeModule(String name) { exitScope(name, this.varsModule); }

  // public void enterScopeRef(String uid, Consumer<Space> refsUpdater) {
  //   this.refsUpdaters.put(uid, refsUpdater);
  // }
  // public void exitScopeRef(String uid) {
  //   this.refsUpdaters.remove(uid);
  // }


  // public HashMap<String, SingleTimeVar> swapVarsST(HashMap<String, SingleTimeVar> varsST) {
  //   HashMap<String, SingleTimeVar> currentVarsST = this.varsST;
  //   this.varsST = varsST;
  //   return currentVarsST;
  // }

  // public Lattice latticeVar(String name, int time) {
  //   Object value = var(name, time);
  //   return Cast.toLattice(name, value);
  // }

  // public void checkWrite(String uid, int time) {
  //   if (time != 0) {
  //     throw new RuntimeException("[BUG] The variable `" + uid
  //       + "` is READ-ONLY because it is surrounded by `pre`.");
  //   }
  // }
}
