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
import bonsai.runtime.core.*;
import inria.meije.rc.sugarcubes.implementation.*;
import inria.meije.rc.sugarcubes.*;

public class SpaceEnvironment extends Clock {
  private HashMap<String, Variable> vars;
  private ArrayDeque<Future> futures;
  private ArrayList<Space> children;
  // When we enter a branch of `space`, we depend on the single time variables of the current instantiated snapshot and not the one of the current environment.
  private SnapshotST currentSnapshotST;
  private boolean inSnapshot;

  // This is updated by the statement `stop` and `pause up`.
  // We need it because the byte status of SugarCubes cannot be easily extended so the status of these statements will be `STOP` (which stands for pause).
  public boolean stopped;
  public boolean pausedUp;

  public SpaceEnvironment(ClockIdentifier clockID,
    InternalIdentifiers anInternalIdentifierGenerator,
    Program body)
  {
    super(clockID, anInternalIdentifierGenerator, body);
    // vars = new HashMap(); // FIXME, cf. declareVar
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

  public void saveFutures() {
    SnapshotWL snapshotWL = new SnapshotWL();
    for (Variable var : vars().values()) {
      var.save(snapshotWL);
    }
    for (int i = children.size() - 1; i >= 0; i--) {
      children.get(i).futures(this, snapshotWL);
    }
    children.clear();
  }

  public void pushFuture(Future future) {
    this.futures.add(future);
  }

  // Precondition: !futures.isEmpty()
  public void instantiateFuture() {
    Future future = futures.pop();
    for(Variable var : vars().values()) {
      var.restore(this, future.snapshotWL);
    }
    currentSnapshotST = future.snapshotST;
    inSnapshot = true;
    future.branch.prepareFor(this);
    future.branch.activate(this);
    inSnapshot = false;
    futureInstantiated = true;
  }

  public void pushSpace(Space space) {
    children.add(space);
  }

  public void enterScope(Variable var) {
    if (var == null) {
      throw new RuntimeException("SpaceEnvironment.enterScope: null `var` parameter.");
    }
    Variable old = vars().put(var.uid(), var);
    if (old != null) {
      throw new RuntimeException(
        "SpaceEnvironment.enterScope: The variable `" + var.name() +
        "` (uid: `" + var.uid() + "`) is already in scope with the name `" + old.name() + "`.");
    }
  }

  public void exitScope(String uid) {
    if (uid == null) {
      throw new RuntimeException("SpaceEnvironment.exitScope: null `uid` parameter.");
    }
    Variable removed = vars().remove(uid);
    if (removed == null) {
      throw new RuntimeException(
        "SpaceEnvironment.enterScope: Try to exit the scope of the variable " +
        "with uid: `" + uid + "` but it is not in scope.");
    }
  }

  public LatticeVar latticeVar(String name, int time, Permission permission) {
    Object value = var(name, time, permission);
    return Cast.toLattice(name, value);
  }

  public void checkVarNull(String uid, Variable v) {
    if (v == null) {
      throw new RuntimeException("The variable `" + uid
        + "` is not registered in the environment.");
    }
  }

  private Object access(String uid, int time, Permission permission) {
    Variable v = vars().get(uid);
    checkVarNull(uid, v);
    // Generate an event on this variable to indicate it might has been modified.
    // Note that `generatePure` does not read the new value of the variable yet–it is just used to wake up suspended statements—so it's OK to generate it after the actual modifications.
    if (time == 0 && permission != Permission.READ) {
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
    return access(uid, 0, Permission.READ);
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
