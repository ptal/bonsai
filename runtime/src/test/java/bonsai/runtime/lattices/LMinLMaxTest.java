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

package bonsai.runtime.lattices;

import bonsai.runtime.core.*;
import java.util.Arrays;
import static org.junit.Assert.*;
import static org.hamcrest.CoreMatchers.*;
import org.junit.*;

public class LMinLMaxTest
{
  LMax bot;
  LMax vm1;
  LMax v0;
  LMax v1;
  LMax v2;
  LMax top;

  LMax[] left;
  LMax[] right;
  Kleene[] entailment;
  LMax[] join;
  LMax[] meet;

  @Before
  public void prepareData() {
    bot = new LMax().bottom();
    vm1 = new LMax(-1);
    v0 = new LMax(0);
    v1 = new LMax(1);
    v2 = new LMax(2);
    top = new LMax().top();

    Kleene T = Kleene.TRUE;
    Kleene F = Kleene.FALSE;
    Kleene U = Kleene.UNKNOWN;

    left =       new LMax[]   { bot, bot, top, bot, top, vm1, v0, v2,  v1 };
    right =      new LMax[]   { top, bot, top, v0,  v0,  v0,  v1, vm1, v1 };
    entailment = new Kleene[] { F,   T,   T,   F,   T,   F,   F,  T,   T  };
    join =       new LMax[]   { top, bot, top, v0,  top, v0,  v1, v2,  v1 };
    meet =       new LMax[]   { bot, bot, top, bot, v0,  vm1, v0, vm1, v1 };
  }

  @Test
  public void testLMaxLattice() {
    LatticeTest test = new LatticeTest();
    test.testLattice("LMax", left, right, entailment, join, meet);
  }

  @Test
  public void testLMinLattice() {
    LatticeTest test = new LatticeTest();
    // We transfer LMax into LMin, reverse the `join` and `meet` data, and negate the expected result from the entailment.
    int n = left.length;
    LMin[] leftMin = new LMin[n];
    LMin[] rightMin = new LMin[n];
    Kleene[] entailmentMin = new Kleene[n];
    LMin[] joinMin = new LMin[n];
    LMin[] meetMin = new LMin[n];

    for(int i = 0; i < left.length; i++) {
      leftMin[i] = new LMin(left[i].unwrap());
      rightMin[i] = new LMin(right[i].unwrap());
      entailmentMin[i] = leftMin[i].equals(rightMin[i]) ? entailment[i] : Kleene.not(entailment[i]);
      joinMin[i] = new LMin(meet[i].unwrap());
      meetMin[i] = new LMin(join[i].unwrap());
    }

    test.testLattice("LMin", leftMin, rightMin, entailmentMin, joinMin, meetMin);
  }

  @Test
  public void testInc() {
    LMax top2 = top.copy();
    top2.inc();
    assertThat("LMax.inc() [1]", top, equalTo(top2));
    LMax x = vm1.copy();
    x.inc();
    assertThat("LMax.inc() [2]", x, equalTo(v0));
    x.inc();
    assertThat("LMax.inc() [3]", x, equalTo(v1));
    bot.inc();
    LMax botPlus1 = new LMax(Integer.MIN_VALUE + 1);
    assertThat("LMax.inc() [4]", bot, equalTo(botPlus1));
  }

  @Test
  public void testDec() {
    LMin top1 = new LMin().top();
    LMin top2 = top1.copy();
    top2.dec();
    assertThat("LMin.dec() [1]", top1, equalTo(top2));
    LMin x = new LMin(v2.unwrap());
    x.dec();
    assertThat("LMin.dec() [2]", x, equalTo(new LMin(v1.unwrap())));
    x.dec();
    assertThat("LMin.dec() [3]", x, equalTo(new LMin(v0.unwrap())));
    LMin bot = new LMin().bottom();
    bot.dec();
    LMin botPlus1 = new LMin(Integer.MAX_VALUE - 1);
    assertThat("LMin.dec() [4]", bot, equalTo(botPlus1));
  }
}
