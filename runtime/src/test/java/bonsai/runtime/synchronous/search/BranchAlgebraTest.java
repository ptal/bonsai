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

public class BranchAlgebraTest
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

  private void testFutures(String name, BranchAlgebra ba, int numFutures) {
    List<Future> futures = ba.unwrap();
    assertThat(name, numFutures, equalTo(futures.size()));
    setUp();
  }

  @Test
  public void testConcat() {
    testFutures("() ; () = ()", n1.concat(n2), 0);
    testFutures("() ; a = a", n1.concat(a), 1);
    testFutures("a ; () = a", a.concat(n1), 1);
    testFutures("a ; b = (a, b)", a.concat(b), 2);
    testFutures("() ; P = ()", n1.concat(p1), 0);
    testFutures("P ; P = ()", p1.concat(p2), 0);
    testFutures("P ; () = ()", p1.concat(n1), 0);
    testFutures("P ; a = a", p1.concat(a), 1);
    testFutures("a ; P = a", a.concat(p1), 1);
    testFutures("P ; a ; b ; (); P = (a,b)", p1.concat(a).concat(b).concat(n1).concat(p2), 2);
  }

  private ArrayList<BranchAlgebra> binary(BranchAlgebra b1, BranchAlgebra b2) {
    return new ArrayList(Arrays.asList(b1, b2));
  }

  public void testPar(String op, Function<ArrayList<BranchAlgebra>, BranchAlgebra> merge) {
    testFutures("() " + op + " () = ()", merge.apply(binary(n1,n2)), 0);
    testFutures("() " + op + " a = a", merge.apply(binary(n1,a)), 1);
    testFutures("a " + op + " () = a", merge.apply(binary(a,n1)), 1);
    testFutures("a " + op + " b = (a " + op + " b)", merge.apply(binary(a,b)), 1);
    testFutures("() " + op + " P = ()", merge.apply(binary(n1,p1)), 0);
    testFutures("P " + op + " P = ()", merge.apply(binary(p1,p2)), 0);
    testFutures("P " + op + " () = ()", merge.apply(binary(p1,n1)), 0);
  }

  @Test
  public void testUnion() {
    testPar("||", BranchAlgebra::union);
    testFutures("P || a = a", BranchAlgebra.union(binary(p1,a)), 1);
    testFutures("a || P = a", BranchAlgebra.union(binary(p1,a)), 1);
    testFutures("P || a || b || () || P = (a || b)", BranchAlgebra.union(
      new ArrayList(Arrays.asList(p1,a,b,n1,p2))), 1);
  }

  @Test
  public void testIntersection() {
    testPar("<>", BranchAlgebra::intersect);
    testFutures("P <> a = ()", BranchAlgebra.intersect(binary(p1,a)), 0);
    testFutures("a <> P = ()", BranchAlgebra.intersect(binary(p1,a)), 0);
    testFutures("P <> a <> b <> () <> P = ()", BranchAlgebra.intersect(
      new ArrayList(Arrays.asList(p1,a,b,n1,p2))), 0);
  }

  @Test
  public void testMultiple() {
    testFutures("a ; (); b || P ; b = (a; b || b)",
      BranchAlgebra.union(binary(a.concat(n1).concat(b), p1.concat(b))), 2);
    testFutures("a ; (); b <> P ; b = (b <> b)",
      BranchAlgebra.intersect(binary(a.concat(n1).concat(b), p1.concat(b))), 1);
  }
}
