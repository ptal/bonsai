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
import java.util.stream.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.statements.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.variables.*;

// `BranchAlgebra` is a sequence of branches that can be pruned or not.
// It contains method to create the neutral element (empty sequence), pruned branch and `space` branch.
// Sequences can be composed by concatenation, union and intersection.
public class BranchAlgebra {
  // `Optional` is empty if we want to `prune` the branch.
  private ArrayList<Optional<Statement>> branches;
  private CapturedSpace space;

  private BranchAlgebra(ArrayList<Optional<Statement>> branches, CapturedSpace space) {
    this.branches = branches;
    this.space = space;
  }

  public static BranchAlgebra neutralElement() {
    return new BranchAlgebra(new ArrayList(), new CapturedSpace());
  }

  public static BranchAlgebra spaceBranch(Statement branch, CapturedSpace space) {
    ArrayList<Optional<Statement>> branches = new ArrayList();
    branches.add(Optional.of(branch));
    return new BranchAlgebra(branches, space);
  }

  public static BranchAlgebra prunedBranch() {
    ArrayList<Optional<Statement>> branches = new ArrayList();
    branches.add(Optional.empty());
    return new BranchAlgebra(branches, new CapturedSpace());
  }

  // Contrarily to `union` and `intersect` the modification are realized in place.
  public BranchAlgebra concat(BranchAlgebra right) {
    branches.addAll(right.branches);
    space.merge(right.space);
    return this;
  }

  private static BranchAlgebra merge(ArrayList<BranchAlgebra> processes,
    BiPredicate<ArrayList<BranchAlgebra>, Integer> isPruned,
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
          if (ba.branches.get(i).isPresent()) {
            parProcesses.add(ba.branches.get(i).get());
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

  public static BranchAlgebra intersect(ArrayList<BranchAlgebra> processes) {
    return BranchAlgebra.merge(processes,
      (ps, i) -> ps.stream().anyMatch(ba -> !ba.branches.get(i).isPresent()),
      (parProcesses) -> new ConjunctivePar(parProcesses, 0)
    );
  }

  public static BranchAlgebra union(ArrayList<BranchAlgebra> processes) {
    return BranchAlgebra.merge(processes,
      (ps, i) -> ps.stream().allMatch(ba -> !ba.branches.get(i).isPresent()),
      (parProcesses) -> new DisjunctivePar(parProcesses, 0)
    );
  }

  private void duplicateLast(int size) {
    while(branches.size() < size) {
      Optional<Statement> last = branches.get(branches.size()-1);
      branches.add(last.map(Statement::copy));
    }
  }

  // Extract the futures of this sequence of branches.
  // It deletes all pruned branches.
  // The branches are removed from the current branch algebra.
  public List<Future> unwrap() {
    List<Future> futures = branches.stream()
      .filter(b -> b.isPresent())
      .map(b -> new Future(b.get(), space))
      .collect(Collectors.toList());
    branches.clear();
    return futures;
  }

  public void registerWL(Variable var, boolean exitScope) {
    space.registerWL(var, exitScope);
  }
}
