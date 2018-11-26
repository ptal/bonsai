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

public class StmtResult {
  public CompletionCode k;
  public HashMap<String, BranchAlgebra> branches;

  public StmtResult(CompletionCode k) {
    this.k = k;
    this.branches = new HashMap();
  }

  public void sequence(StmtResult res) {
    k = res.k;
    for (Map.Entry<String, BranchAlgebra> entry : res.branches.entrySet()) {
      branches.merge(entry.getKey(), entry.getValue(), BranchAlgebra::concat);
    }
  }

  // We do not merge the branches if one completion is still internal.
  private static StmtResult par(ArrayList<StmtResult> processes,
   Function<List<BranchAlgebra>, BranchAlgebra> join)
  {
    HashSet<String> queues = new HashSet();
    StmtResult res = new StmtResult(CompletionCode.TERMINATE);
    for(StmtResult process : processes) {
      res.k = res.k.merge(process.k);
      queues.addAll(process.branches.keySet());
    }
    if (!res.k.isInternal()) {
      for(String queue: queues) {
        List<BranchAlgebra> bas = processes.stream()
          .map(p -> p.branches.get(queue))
          .filter(ba -> ba != null)
          .collect(Collectors.toList());
        res.branches.put(queue, join.apply(bas));
      }
    }
    return res;
  }

  public void conjunctivePar(ArrayList<StmtResult> res) {
    par(res, BranchAlgebra::intersect);
  }

  public void disjunctivePar(ArrayList<StmtResult> res) {
    par(res, BranchAlgebra::union);
  }
}
