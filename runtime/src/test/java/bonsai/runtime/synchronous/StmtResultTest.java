// Copyright 2018 Pierre Talbot (IRCAM)

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
import bonsai.runtime.lattices.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.statements.*;
import bonsai.runtime.synchronous.search.*;
import bonsai.runtime.synchronous.expressions.*;
import bonsai.runtime.synchronous.interfaces.*;
import static org.junit.Assert.*;
import static org.hamcrest.CoreMatchers.*;
import org.junit.*;

public class StmtResultTest
{
  protected String currentTest;
  private BranchAlgebra n1;
  private BranchAlgebra n2;
  private BranchAlgebra a;
  private BranchAlgebra b;
  private BranchAlgebra p1;
  private BranchAlgebra p2;

  @Before
  public void setUp() {
    n1 = BranchAlgebra.neutralElement();
    n2 = BranchAlgebra.neutralElement();
    a = BranchAlgebra.spaceBranch(new Nothing(), new CapturedSpace());
    b = BranchAlgebra.spaceBranch(new Nothing(), new CapturedSpace());
    p1 = BranchAlgebra.prunedBranch();
    p2 = BranchAlgebra.prunedBranch();
  }

  private void testFuturesPerQueue(String name, StmtResult res, HashMap<String, Integer> futuresPerQueue) {
    HashMap<String, List<Future>> futures = res.unwrap();
    for(Map.Entry<String, List<Future>> future : futures.entrySet()) {
      assertThat(name, futuresPerQueue.get(future.getKey()), equalTo(future.getValue().size()));
    }
    setUp();
  }

  private StmtResult make(String queue, BranchAlgebra ba) {
    return new StmtResult(CompletionCode.TERMINATE, queue, ba);
  }

  private ArrayList<StmtResult> binary(StmtResult b1, StmtResult b2) {
    return new ArrayList(Arrays.asList(b1, b2));
  }

  @Test
  public void testAll() {
    StmtResult q1p1 = make("q1", p1);
    StmtResult q1p2 = make("q1", p2);
    StmtResult q1a = make("q1", a);
    StmtResult q1b = make("q1", b);
    StmtResult q3n1 = make("q3", n1);
    StmtResult q3b = make("q3", b);
    StmtResult q3p2 = make("q3", p2);

    //   ((universe with q1 in prune ; space a) || (universe with q3 space b))
    // <>((universe with q1 in space b ; prune) || (universe with q3 ()))

    StmtResult res = StmtResult.conjunctivePar(binary(
      StmtResult.disjunctivePar(binary(q1p1.sequence(q1a), q3b)),
      StmtResult.disjunctivePar(binary(q1b.sequence(q1p2), q3n1))));

    HashMap<String, Integer> futuresPerQueue = new HashMap();
    futuresPerQueue.put("q1", 0);
    futuresPerQueue.put("q3", 1);

    testFuturesPerQueue("mixed 1", res, futuresPerQueue);
  }
}
