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
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.variables.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.statements.SpaceStmt;

public class SpaceEnvironment<Queue extends Queueing<Future>>
{
  private HashMap<String, SingleTimeVar> varsST;
  private HashMap<String, StreamVar> varsSS;
  private HashMap<String, StreamVar> varsWL;
  private HashMap<String, ModuleVar> varsModule;
  private HashMap<String, Consumer<SpaceEnvironment>> refsUpdaters;

  private Queue queue;

  private HashMap<String, RWCounter> monotonic_counters;
  private HashMap<String, RWCounter> anti_monotonic_counters;
  private HashMap<String, ArrayList<Program>> waiting_queue;

  class RWCounter {
    public int write;
    public int readwrite;
    public RWCounter(int write, int readwrite) {
      this.write = write;
      this.readwrite = readwrite;
    }
  }

  public SpaceEnvironment(Queue queue)
  {
    varsST = new HashMap();
    varsSS = new HashMap();
    varsWL = new HashMap();
    varsModule = new HashMap();
    refsUpdaters = new HashMap();
    this.queue = queue;
    monotonic_counters = new HashMap();
    anti_monotonic_counters = new HashMap();
    waiting_queue = new HashMap();
  }

  /// We create and push the branches created in the current instant onto the queue.
  /// It corresponds to `push: Store(L3) -> L4` in the lattice hierarchy.
  /// Caution: this function must always be called even with an empty set of branches.
  /// Rational: this function marks the end of an instant, and therefore internal data might be reinitialized.
  /// @see Future
  public void pushBranches(ArrayList<SpaceStmt> branches) {
    ArrayList<Future> futures = Future.createFutures(branches, varsST, varsWL);
    queue.push(futures);
  }

  public Optional<Future> pop() {
    if(!queue.isEmpty()) {
      Future future = queue.pop();
      future.restoreWL(varsWL);
      return Optional.of(future);
    }
    else {
      return Optional.empty();
    }
  }

  public HashMap<String, SingleTimeVar> swapVarsST(HashMap<String, SingleTimeVar> varsST) {
    HashMap<String, SingleTimeVar> currentVarsST = this.varsST;
    this.varsST = varsST;
    return currentVarsST;
  }

  private <T extends Variable> void enterScope(T var, HashMap<String, T> vars) {
    if (var == null) {
      throw new RuntimeException("SpaceEnvironment.enterScope: null `var` parameter.");
    }
    T old = vars.put(var.uid(), var);
    if (old != null) {
      throw new RuntimeException(
        "SpaceEnvironment.enterScope: The variable `" + var.name() +
        "` (uid: `" + var.uid() + "`) is already in scope with the name `" + old.name() + "`.");
    }
  }

  private <T extends Variable> void exitScope(String uid, HashMap<String, T> vars) {
    if (uid == null) {
      throw new RuntimeException("SpaceEnvironment.exitScope: null `uid` parameter.");
    }
    Variable removed = vars.remove(uid);
    if (removed == null) {
      throw new RuntimeException(
        "SpaceEnvironment.exitScope: Try to exit the scope of the variable " +
        "with uid: `" + uid + "` but it is not in scope.");
    }
  }

  public void enterScopeST(SingleTimeVar var) { enterScope(var, this.varsST); }
  public void enterScopeSS(StreamVar var) { enterScope(var, this.varsSS); }
  public void enterScopeWL(StreamVar var) { enterScope(var, this.varsWL); }
  public void enterScopeModule(ModuleVar var) { enterScope(var, this.varsModule); }
  public void exitScopeST(String uid) { exitScope(uid, this.varsST); }
  public void exitScopeSS(String uid) { exitScope(uid, this.varsSS); }
  public void exitScopeWL(String uid) { exitScope(uid, this.varsWL); }
  public void exitScopeModule(String uid) { exitScope(uid, this.varsModule); }

  public void enterScopeRef(String uid, Consumer<SpaceEnvironment> refsUpdater) {
    this.refsUpdaters.put(uid, refsUpdater);
  }
  public void exitScopeRef(String uid) {
    this.refsUpdaters.remove(uid);
  }

  public Lattice latticeVar(String name, int time) {
    Object value = var(name, time);
    return Cast.toLattice(name, value);
  }

  public void checkVarNull(String uid, Variable v) {
    if (v == null) {
      throw new RuntimeException("The variable `" + uid
        + "` is not registered in the environment.");
    }
  }

  public void checkWrite(String uid, int time) {
    if (time != 0) {
      throw new RuntimeException("[BUG] The variable `" + uid
        + "` is READ-ONLY because it is surrounded by `pre`.");
    }
  }

  private Object access(String uid, int time) {
    return null;
    // Variable v = vars().get(uid);
    // checkVarNull(uid, v);
    // Generate an event on this variable to indicate it might has been modified.
    // Note that `generatePure` does not read the new value of the variable yet–it is just used to wake up suspended statements—so it's OK to generate it before the actual modifications.
    // if (permission != Permission.READ) {
    //   checkWrite(uid, time);
    //   Event event = getDirectAccessToEvent(new StringID(uid));
    //   event.generatePure(this);
    // }
    // return v.value(time);
  }

  public Object var(String uid, int time) {
    return access(uid, time);
  }

  public Object module(String uid) {
    return varsModule.get(uid);
  }

  public boolean isEmpty() {
    return queue.isEmpty();
  }

  public int queueSize() {
    return queue.size();
  }

  // private HashMap<String, Variable> vars() {
  //   return vars;
  // }
}
