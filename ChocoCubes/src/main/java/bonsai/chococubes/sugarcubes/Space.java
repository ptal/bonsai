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

package bonsai.chococubes.sugarcubes;

import java.util.*;
import bonsai.chococubes.core.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class Space extends Atom
{
  private ArrayList<SpaceBranch> branches;
  private ArrayList<Integer> branchesIndexes;

  public Space(ArrayList<SpaceBranch> branches) {
    super();
    this.branches = branches;
    branchesIndexes = new ArrayList();
  }

  public String actualToString() {
    return branches.stream()
      .map((b) -> b.toString())
      .reduce("space\n", (accu, b) -> new String(accu + "|| " + b))
      + " end\n";
  }

  public Instruction copy() {
    ArrayList<SpaceBranch> branchesCopy = new ArrayList();
    for (SpaceBranch b : branches) {
      branchesCopy.add((SpaceBranch) b.copy());
    }
    return new Space(branchesCopy);
  }

  public Instruction prepareFor(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    for (SpaceBranch branch : branches) {
      Integer branchIndex = env.registerSpaceBranch(branch);
      branchesIndexes.add(branchIndex);
      branch.setParent(this);
    }
    return this;
  }

  public boolean action(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    env.activateSpace(branchesIndexes);
    return false;
  }
}
