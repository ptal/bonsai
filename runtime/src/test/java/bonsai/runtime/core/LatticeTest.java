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

package bonsai.runtime.core;

import static org.junit.Assert.*;
import static org.hamcrest.CoreMatchers.*;
import org.junit.*;

public class LatticeTest
{
  private String currentTest;

  // a.entail(b) == TRUE => b.entail(a) != UNKNOWN
  private void testEntailmentTrue(Lattice a, Lattice b) {
    Kleene ba = b.entail(a);
    assertThat(currentTest, ba, not(equalTo(Kleene.UNKNOWN)));
    switch (ba) {
      case TRUE: {
        testEquality(a, b, true);
        break;
      }
      case FALSE: {
        testStrictEntail(a, b, Kleene.TRUE);
        break;
      }
      default: {
        testStrictEntail(a, b, Kleene.UNKNOWN);
        break;
      }
    }
  }

  // a.entail(b) == FALSE => b.entail(a) == TRUE
  private void testEntailmentFalse(Lattice a, Lattice b) {
    Kleene ba = b.entail(a);
    assertThat(currentTest, ba, equalTo(Kleene.TRUE));
    testEquality(a, b, false);
  }

  // a.entail(b) == UNKNOWN => b.entail(a) == UNKNOWN
  private void testEntailmentUnknown(Lattice a, Lattice b) {
    Kleene ba = b.entail(a);
    assertThat(currentTest, ba, equalTo(Kleene.UNKNOWN));
    testEquality(a, b, false);
  }

  private void testEntailment(Lattice a, Lattice b, Kleene expected) {
    Kleene ab = a.entail(b);
    assertThat(currentTest, ab, equalTo(expected));
    switch (expected) {
      case TRUE: {
        testEntailmentTrue(a, b);
        break;
      }
      case UNKNOWN: {
        testEntailmentUnknown(a, b);
        break;
      }
      default: {
        testEntailmentFalse(a, b);
        break;
      }
    }
  }

  private void testStrictEntail(Lattice a, Lattice b, Kleene expected) {
    Kleene ab = a.strict_entail(b);
    Kleene ba = b.strict_entail(a);
    assertThat(currentTest, ab, equalTo(expected));
    assertThat(currentTest, ba, equalTo(Kleene.not(expected)));
    switch (expected) {
      case TRUE:
      case UNKNOWN: {
        testEquality(a, b, false);
        break;
      }
      default: break;
    }
  }

  private void testEquality(Lattice a, Lattice b, boolean expected) {
    assertThat(currentTest, a.equals(b), equalTo(expected));
    assertThat(currentTest, b.equals(a), equalTo(expected));
  }

  private void testJoin(Lattice a, Lattice b, Lattice expected) {
    Lattice c = a.join(b);
    testEquality(c, expected, true);

    // Test commutativity.
    Lattice d = b.join(a);
    testEquality(c, d, true);

    // Test idempotence.
    Lattice e = a.join(b).join(c);
    testEquality(c, e, true);
    Lattice f = d.join(c);
    testEquality(c, f, true);
    Lattice g = a.join(a);
    Lattice h = b.join(b);
    testEquality(a, g, true);
    testEquality(b, h, true);

    // Test relation with entailment.
    testEntailment(c, a, Kleene.TRUE);
    testEntailment(c, b, Kleene.TRUE);

    if (a.equals(c)) {
      testEntailment(a, b, Kleene.TRUE);
    }
    if (b.equals(c)) {
      testEntailment(b, a, Kleene.TRUE);
    }
  }

  private void testMeet(Lattice a, Lattice b, Lattice expected) {
    Lattice c = a.meet(b);
    testEquality(c, expected, true);

    // Test commutativity.
    Lattice d = b.meet(a);
    testEquality(c, d, true);

    // Test idempotence.
    Lattice e = a.meet(b).meet(c);
    testEquality(c, e, true);
    Lattice f = d.meet(c);
    testEquality(c, f, true);
    Lattice g = a.meet(a);
    Lattice h = b.meet(b);
    testEquality(a, g, true);
    testEquality(b, h, true);

    // Test relation with entailment.
    testEntailment(a, c, Kleene.TRUE);
    testEntailment(b, c, Kleene.TRUE);

    if (a.equals(c)) {
      testEntailment(b, a, Kleene.TRUE);
    }
    if (b.equals(c)) {
      testEntailment(a, b, Kleene.TRUE);
    }
  }

  public void testLattice(String testID, Lattice[] a, Lattice[] b,
    Kleene[] expectedEntailment,
    Lattice[] expectedJoin,
    Lattice[] expectedMeet)
  {
    assert a.length == b.length;
    assert b.length >= expectedEntailment.length;
    assert b.length >= expectedJoin.length;
    assert b.length >= expectedMeet.length;
    int testNo = 0;
    for(int i = 0; i < expectedEntailment.length; i++) {
      currentTest = testID + ".entail[" + testNo + "]";
      testEntailment(a[i], b[i], expectedEntailment[i]);
      testNo = testNo + 1;
    }
    for(int i = 0; i < expectedJoin.length; i++) {
      currentTest = testID + ".join[" + testNo + "]";
      testJoin(a[i], b[i], expectedJoin[i]);
      testNo = testNo + 1;
    }
    for(int i = 0; i < expectedMeet.length; i++) {
      currentTest = testID + ".meet[" + testNo + "]";
      testMeet(a[i], b[i], expectedMeet[i]);
      testNo = testNo + 1;
    }
  }
}
