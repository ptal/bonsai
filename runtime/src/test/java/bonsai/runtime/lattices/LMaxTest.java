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
import static org.junit.Assert.*;
import static org.hamcrest.CoreMatchers.*;
import org.junit.*;

public class LMaxTest
{
  LMax bot;
  LMax vm1;
  LMax v0;
  LMax v1;
  LMax v2;
  LMax top;

  @Before
  public void initLMax() {
    bot = new LMax().bottom();
    vm1 = new LMax(-1);
    v0 = new LMax(0);
    v1 = new LMax(1);
    v2 = new LMax(2);
    top = new LMax().top();
  }

  @Test
  public void testLattice() {
    LatticeTest test = new LatticeTest();

    Kleene T = Kleene.TRUE;
    Kleene F = Kleene.FALSE;
    Kleene U = Kleene.UNKNOWN;

    test.testLattice("LMax",
      new LMax[]   { bot, bot, top, bot, top, vm1, v0, v2,  v1 },
      new LMax[]   { top, bot, top, v0,  v0,  v0,  v1, vm1, v1 },
      new Kleene[] { F,   T,   T,   F,   T,   F,   F,  T,   T  }, // entailment
      new LMax[]   { top, bot, top, v0,  top, v0,  v1, v2,  v1 }, // join
      new LMax[]   { bot, bot, top, bot, v0,  vm1, v0, vm1, v1 }  // meet
    );
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
}
