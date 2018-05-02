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

package bonsai.runtime.sugarcubes;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.core.*;
import inria.meije.rc.sugarcubes.implementation.*;
import inria.meije.rc.sugarcubes.*;

public class SpaceEnvironment extends Clock {
  private HashMap<String, SingleTimeVar> varsST;
  private HashMap<String, StreamVar> varsSS;
  private HashMap<String, StreamVar> varsWL;
  private HashMap<String, ModuleVar> varsModule;
  private HashMap<String, Consumer<SpaceEnvironment>> refsUpdaters;

  private ArrayDeque<Future> futures;
  private ArrayList<Space> children;

  // This is updated by the statement `stop` and `pause up`.
  // We need it because the byte status of SugarCubes cannot be easily extended so the status of these statements will be `STOP` (which stands for pause).
  public boolean stopped;
  public boolean pausedUp;

  public SpaceEnvironment(ClockIdentifier clockID,
    InternalIdentifiers anInternalIdentifierGenerator,
    Program body)
  {
    super(clockID, anInternalIdentifierGenerator, body);
    varsST = new HashMap();
    varsSS = new HashMap();
    varsWL = new HashMap();
    varsModule = new HashMap();
    refsUpdaters = new HashMap();
    futures = new ArrayDeque();
    children = new ArrayList();
    currentSnapshotST = null;
    inSnapshot = false;
    resetFlags();
  }

  private boolean firstActivation = true;
  public byte activation(Environment env) {
    if (beginingOfInstant) {
      if (firstActivation) {
        firstActivation = false;
      }
      else {
        if (!futureInstantiated) {
          if (futures.isEmpty()) {
            return TERM;
          }
          else {
            instantiateFuture();
          }
        }
      }
    }
    return super.activation(env);
  }

  // See SpaceMachine.commit()
  private boolean futureInstantiated;
  public boolean commit() {
    if (!futureInstantiated && !futures.isEmpty()) {
      instantiateFuture();
      return true;
    }
    return false;
  }

  public void resetFlags() {
    stopped = false;
    pausedUp = false;
    futureInstantiated = false;
  }

  // Big step transition.
  public void newInstant() {
    saveFutures();
    futureInstantiated = false;
    super.newInstant();
  }

  /// For each branch of each space, we create a future.
  /// See class `Future` for explanations on snapshots.
  public void saveFutures() {
    HashMap<String, Object> snapshotWL = snapshotWL();
    for (int i = children.size() - 1; i >= 0; i--) {
      Space child = children.get(i);
      HashMap<String, SingleTimeVar> snapshotST = snapshotST(child);
      ArrayList<SpaceBranch> branches = child.branches();
      for (int i = branches.size()-1; i >= 0; i--) {
        futures.add(new Future(branches.get(i), snapshotST, snapshotWL));
      }
    }
    children.clear();
  }

  private HashMap<String, Object> snapshotWL() {
    HashMap<String, Object> snapshotWL = new HashMap();
    for (StreamVar var : varsWL.values()) {
      snapshotWL.put(var.uid(), var.stream().label());
    }
    return snapshotWL;
  }

  private HashMap<String, SingleTimeVar> snapshotST(Space child) {
    HashMap<String, SingleTimeVar> snapshotST = new HashMap();
    for (String varUID: child.singleTimeClosure()) {
      snapshotST.put(varUID, varsST.get(varUID));
    }
    return snapshotST;
  }

  // Precondition: !futures.isEmpty()
  public void instantiateFuture() {
    Future future = futures.pop();
    for(StreamVar var : varsWL.values()) {
      var.stream().restore(future.snapshotWL.get(uid));
    }
    HashMap<String, SingleTimeVar> current = varsST;
    varsST = future.snapshotST;
    future.branch.prepareFor(this);
    future.branch.activate(this);
    varsST = current;
    futureInstantiated = true;
  }

  public void pushSpace(Space space) {
    children.add(space);
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
  public void enterScopeSS(SingleSpaceVar var) { enterScope(var, this.varsSS); }
  public void enterScopeWL(WorldLineVar var) { enterScope(var, this.varsWL); }
  public void enterScopeModule(ModuleVar var) { enterScope(var, this.varsModule); }
  public void exitScopeST(String uid) { exitScope(uid, this.varsST); }
  public void exitScopeSS(String uid) { exitScope(uid, this.varsSS); }
  public void exitScopeWL(String uid) { exitScope(uid, this.varsWL); }
  public void exitScopeModule(String uid) { exitScope(uid, this.varsModule); }

  public void enterScopeRef(String uid, Consumer<SpaceEnvironment> refsUpdater) {
    this.refsUpdaters.put(uid, refsUpdater);
  }
  public void exitScopeRef(String uid) {
    this.refsUpdaters.pop(uid);
  }

  public Lattice latticeVar(String name, int time, Permission permission) {
    Object value = var(name, time, permission);
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

  private Object access(String uid, int time, Permission permission) {
    Variable v = vars().get(uid);
    checkVarNull(uid, v);
    // Generate an event on this variable to indicate it might has been modified.
    // Note that `generatePure` does not read the new value of the variable yet–it is just used to wake up suspended statements—so it's OK to generate it before the actual modifications.
    if (permission != Permission.READ) {
      checkWrite(uid, time);
      Event event = getDirectAccessToEvent(new StringID(uid));
      event.generatePure(this);
    }
    return v.value(time);
  }

  public Object var(String uid, int time, Permission permission) {
    if (inSnapshot) {
      Optional<Object> value = currentSnapshotST.getSingleTimeValue(uid);
      if (value.isPresent()) {
        return value.get();
      }
    }
    return access(uid, time, permission);
  }

  public Object module(String uid) {
    return varsModule.get(uid);
  }

  public boolean isEmpty() {
    return futures.isEmpty();
  }

  public int queueSize() {
    return futures.size();
  }

  private HashMap<String, Variable> vars() {
    // FIXME: `vars` can be null because `SpaceEnvironment` is used in `Instruction.prepareFor()` but this same instruction is called from the constructor of Clock.
    if (vars == null) {
      vars = new HashMap();
    }
    return vars;
  }
}
