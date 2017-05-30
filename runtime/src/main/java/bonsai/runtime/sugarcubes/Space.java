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

/// `Space` represents the spacetime statement `space b1 || ... || bn end`.
/// The fields `branches` is the code of the branches `{b1,...,bn}` and `singleTimeClosure` contains the variables annotated with `single_time` captured in any of these branches.

package bonsai.runtime.sugarcubes;

import java.util.*;
import bonsai.runtime.core.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class Space extends Atom
{
  private ArrayList<String> singleTimeClosure;
  private ArrayList<SpaceBranch> branches;

  public Space(ArrayList<String> singleTimeClosure, ArrayList<SpaceBranch> branches) {
    super();
    this.singleTimeClosure = singleTimeClosure;
    this.branches = branches;
  }

  public String actualToString() {
    return branches.stream()
      .map((b) -> b.toString())
      .reduce("space\n", (accu, b) -> new String(accu + "|| " + b))
      + " end\n";
  }

  public Space copy() {
    ArrayList<SpaceBranch> branchesCopy = new ArrayList();
    for (SpaceBranch b : branches) {
      branchesCopy.add((SpaceBranch) b.copy());
    }
    return new Space(
      (ArrayList<String>) singleTimeClosure.clone(),
      branchesCopy);
  }

  public Space prepareFor(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    for (SpaceBranch branch : branches) {
      branch.setParent(this);
    }
    return this;
  }

  public boolean action(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    env.pushSpace(this);
    return false;
  }

  public void futures(SpaceEnvironment env, SnapshotWL snapshotWL) {
    SnapshotST snapshotST = new SnapshotST();
    for (String varUID: singleTimeClosure) {
      Object value = env.var(varUID, 0, Permission.READ);
      snapshotST.saveSingleTimeVar(varUID, value);
    }
    for (int i = branches.size()-1; i >= 0; i--) {
      env.pushFuture(new Future(branches.get(i), snapshotST, snapshotWL));
    }
  }
}
