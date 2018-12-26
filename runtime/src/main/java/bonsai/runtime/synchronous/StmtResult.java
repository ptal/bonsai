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
import java.util.stream.*;
import java.util.function.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.search.*;
import bonsai.runtime.synchronous.variables.*;

public class StmtResult {
  public CompletionCode k;
  public HashMap<String, BranchAlgebra> branchesPerQueue;

  public StmtResult(CompletionCode k) {
    this.k = k;
    this.branchesPerQueue = new HashMap();
  }

  public StmtResult(CompletionCode k, String queue, BranchAlgebra ba) {
    this(k);
    branchesPerQueue.put(queue, ba);
  }

  public void registerWL(String queue, Variable var, boolean exitScope) {
    BranchAlgebra ba = branchesPerQueue.computeIfAbsent(queue, q -> BranchAlgebra.neutralElement());
    ba.registerWL(var, exitScope);
  }

  public StmtResult sequence(StmtResult res) {
    k = res.k;
    for (Map.Entry<String, BranchAlgebra> entry : res.branchesPerQueue.entrySet()) {
      branchesPerQueue.merge(entry.getKey(), entry.getValue(), BranchAlgebra::concat);
    }
    res.branchesPerQueue.clear();
    return this;
  }

  // We do not merge the branches if one completion is still internal.
  private static StmtResult par(ArrayList<StmtResult> processes,
   Function<ArrayList<BranchAlgebra>, BranchAlgebra> join)
  {
    HashSet<String> queues = new HashSet();
    StmtResult res = new StmtResult(CompletionCode.TERMINATE);
    for(StmtResult process : processes) {
      res.k = res.k.merge(process.k);
      queues.addAll(process.branchesPerQueue.keySet());
    }
    if (!res.k.isInternal()) {
      for(String queue: queues) {
        ArrayList<BranchAlgebra> bas = processes.stream()
          .map(p -> p.branchesPerQueue.get(queue))
          .filter(ba -> ba != null)
          .collect(Collectors.toCollection(ArrayList::new));
        res.branchesPerQueue.put(queue, join.apply(bas));
      }
      processes.stream().forEach(p -> p.branchesPerQueue.clear());
    }
    return res;
  }

  public static StmtResult conjunctivePar(ArrayList<StmtResult> res) {
    return par(res, BranchAlgebra::intersect);
  }

  public static StmtResult disjunctivePar(ArrayList<StmtResult> res) {
    return par(res, BranchAlgebra::union);
  }

  // See also `BranchAlgebra.unwrap()`.
  public HashMap<String, List<Future>> unwrap() {
    HashMap<String, List<Future>> futuresPerQueue = new HashMap();
    for(Map.Entry<String, BranchAlgebra> entry : branchesPerQueue.entrySet()) {
      futuresPerQueue.put(entry.getKey(), entry.getValue().unwrap());
    }
    branchesPerQueue.clear();
    return futuresPerQueue;
  }
}
