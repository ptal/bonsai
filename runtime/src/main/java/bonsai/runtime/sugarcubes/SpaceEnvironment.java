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
  private HashMap<String, SpacetimeVar> vars;
  private ArrayDeque<Snapshot> futures;
  private ArrayList<SpaceBranch> branches;
  private ArrayList<Integer> activatedBranches;
  // When we enter a branch of `space`, we depend on the single time variables of the current instantiated snapshot and not the one of the current environment.
  private Snapshot currentSnapshot;
  private SpaceBranch currentBranch;
  private boolean inSnapshot;

  public SpaceEnvironment(ClockIdentifier clockID,
    InternalIdentifiers anInternalIdentifierGenerator,
    Program body)
  {
    super(clockID, anInternalIdentifierGenerator, body);
    // vars = new HashMap(); // FIXME, cf. declareVar
    futures = new ArrayDeque();
    // branches = new ArrayList(); // FIXME, cf. registerSpaceBranch
    activatedBranches = new ArrayList();
    currentSnapshot = null;
    // FIXME, the current branch should be incorporated into the current running program. For example, one of the problem is that it does not call activateOnEOI.
    currentBranch = null;
    inSnapshot = false;
  }

  private boolean firstActivation = true;
  public byte activation(Environment env) {
    if (beginingOfInstant) {
      if (firstActivation) {
        firstActivation = false;
      }
      else {
        if (futures.isEmpty()) {
          return TERM;
        }
        else {
          instantiateFuture();
        }
      }
    }
    if (currentBranch != null) {
      currentBranch.activate(this);
    }
    return super.activation(env);
  }

  // Big step transition.
  public void newInstant() {
    saveFutures();
    super.newInstant();

  }

  public void saveFutures() {
    for (Integer branchIdx: activatedBranches) {
      Snapshot future = new Snapshot(branchIdx);
      for (SpacetimeVar var : vars.values()) {
        var.save(future);
      }
      futures.push(future);
    }
    activatedBranches.clear();
  }

  // Precondition: !futures.isEmpty()
  public void instantiateFuture() {
    currentSnapshot = futures.pop();
    for(Map.Entry<String, SpacetimeVar> var : vars.entrySet()) {
      var.getValue().restore(this, currentSnapshot);
    }
    int b = currentSnapshot.branch();
    currentBranch = branches.get(b);
    currentBranch.prepareFor(this);
  }

  // For shadowing the single time variables when executing a branch.
  public void enterSpaceBranch() {
    if (currentSnapshot == null) {
      throw new RuntimeException(
        "Cannot enter a space branch without a snapshot previously installed.");
    }
    inSnapshot = true;
  }

  public void exitSpaceBranch() {
    inSnapshot = false;
  }

  public Integer registerSpaceBranch(SpaceBranch branch) {
    // FIXME: branches can be null because `SpaceEnvironment` is used in prepareFor of instructions but prepareFor is called in the constructor of Clock.
    if (branches == null) {
      branches = new ArrayList();
    }
    branches.add(branch);
    return branches.size() - 1;
  }

  // At the end of the current instant, the branches at `branchesIndexes` will be turned into `Snapshot` for future activation.
  public void activateSpace(ArrayList<Integer> branchesIndexes) {
    for (Integer idx : branchesIndexes) {
      if (idx >= branches.size()) {
        throw new RuntimeException(
          "activateSpace: Try to activate a undeclared or not existing space.");
      }
      activatedBranches.add(idx);
    }
  }

  public void declareVar(String name, SpacetimeVar v) {
    // FIXME: similar problem than with `registerSpaceBranch`.
    if (vars == null) {
      vars = new HashMap();
    }
    vars.put(name, v);
  }

  public LatticeVar latticeVar(String name) {
    Object value = var(name);
    if (!(value instanceof LatticeVar)) {
      throw new RuntimeException(
        "Try to use `v <- e` or `v |= e` on a variable that do not implement LatticeVar.");
    }
    return (LatticeVar) value;
  }

  public Object var(String name) {
    if (inSnapshot) {
      Optional<Object> value = currentSnapshot.getSingleTimeValue(name);
      if (value.isPresent()) {
        return value.get();
      }
    }
    SpacetimeVar v = vars.get(name);
    if (v == null) {
      throw new RuntimeException("The variable " + name + " is not registered in the environment.");
    }
    return v.value();
  }

  public boolean isEmpty() {
    return futures.isEmpty();
  }

  public int queueSize() {
    return futures.size();
  }
}
