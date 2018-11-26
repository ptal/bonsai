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
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.statements.*;
import bonsai.runtime.synchronous.interfaces.*;

// `BranchAlgebra` is a sequence of branches that can be pruned or not.
// It contains method to create the neutral element (empty sequence), pruned branch and `space` branch.
// Sequences can be composed by concatenation, union and intersection.
public class BranchAlgebra {
  // `null` is equal to `prune`.
  private static final Statement PRUNED = null;
  private ArrayList<Statement> branches;
  private CapturedSpace space;

  private BranchAlgebra(ArrayList<Statement> branches, CapturedSpace space) {
    this.branches = branches;
    this.space = space;
  }

  public static BranchAlgebra neutralElement() {
    return new BranchAlgebra(new ArrayList(), new CapturedSpace());
  }

  public static BranchAlgebra spaceBranch(Statement branch, CapturedSpace space) {
    ArrayList<Statement> branches = new ArrayList();
    branches.add(branch);
    return new BranchAlgebra(branches, space);
  }

  public static BranchAlgebra prunedBranch() {
    ArrayList<Statement> branches = new ArrayList();
    branches.add(PRUNED);
    return new BranchAlgebra(branches, new CapturedSpace());
  }

  public BranchAlgebra concat(BranchAlgebra right) {
    branches.addAll(right.branches);
    space.merge(right.space);
    return this;
  }

  private static BranchAlgebra merge(List<BranchAlgebra> processes,
    BiPredicate<List<BranchAlgebra>, Integer> isPruned,
    Function<ArrayList<Statement>, Statement> parStatement)
  {
    // We remove neutral branch algebras.
    for(int i=0; i < processes.size(); i++) {
      if(processes.get(i).branches.isEmpty()) {
        processes.remove(i);
        i--;
      }
    }
    // We ensure that all processes have the same numbers of branches.
    int max = 0;
    for(BranchAlgebra ba: processes) {
      max = Math.max(max, ba.branches.size());
    }
    for(BranchAlgebra ba: processes) {
      ba.duplicateLast(max);
    }
    // We merge the remaining branches, it proceeds as follows:
    // Iteration 1:  a; b; c || a'; b'; c'
    //               ^          ^
    // Iteration 2:  a; b; c || a'; b'; c'
    //                  ^           ^
    // Iteration 3:  a; b; c || a'; b'; c'
    //                     ^            ^
    // with an arbitrary number of parallel branches.
    BranchAlgebra res = BranchAlgebra.neutralElement();
    for(int i=0; i < max; i++) {
      if (isPruned.test(processes, i)) {
        res.concat(BranchAlgebra.prunedBranch());
      }
      else {
        ArrayList<Statement> parProcesses = new ArrayList();
        CapturedSpace parSpace = new CapturedSpace();
        for(BranchAlgebra ba: processes) {
          // We ignore the pruned branches.
          if (ba.branches.get(i) != PRUNED) {
            parProcesses.add(ba.branches.get(i));
            parSpace.merge(ba.space);
          }
        }
        res.concat(BranchAlgebra.spaceBranch(
          parStatement.apply(parProcesses),
          parSpace
        ));
      }
    }
    return res;
  }

  public static BranchAlgebra intersect(List<BranchAlgebra> processes) {
    return BranchAlgebra.merge(processes,
      (ps, i) -> ps.stream().anyMatch(ba -> ba.branches.get(i) == PRUNED),
      (parProcesses) -> new ConjunctivePar(parProcesses)
    );
  }

  public static BranchAlgebra union(List<BranchAlgebra> processes) {
    return BranchAlgebra.merge(processes,
      (ps, i) -> ps.stream().allMatch(ba -> ba.branches.get(i) == PRUNED),
      (parProcesses) -> new DisjunctivePar(parProcesses)
    );
  }

  private void duplicateLast(int size) {
    while(branches.size() < size) {
      Statement last = branches.get(branches.size()-1);
      if (last != PRUNED) {
        last = last.copy();
      }
      branches.add(last);
    }
  }
}
