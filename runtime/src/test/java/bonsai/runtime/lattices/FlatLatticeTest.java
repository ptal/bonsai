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

public class FlatLatticeTest
{
  L<Integer> bot;
  L<Integer> top;
  L<Integer> v0;
  L<Integer> v1;
  L<Integer> v2;

  @Before
  public void prepareData() {
    bot = new L<Integer>().bottom();
    top = new L<Integer>().top();
    v0 = new L<Integer>(0);
    v1 = new L<Integer>(1);
    v2 = new L<Integer>(2);
  }

  @Test
  public void testFlatLattice() {
    LatticeTest test = new LatticeTest();

    Kleene T = Kleene.TRUE;
    Kleene F = Kleene.FALSE;
    Kleene U = Kleene.UNKNOWN;

    test.testLattice("L<Integer>",
      new L[]     { bot, bot, top, bot, top, v0, v0,  v2, top },
      new L[]     { bot, top, top, v0,  v0,  v0, v1,  v0, v2  },
      new Kleene[]{ T,   F,   T,   F,   T,   T,  U,   U,  T   }, // entailment
      new L[]     { bot, top, top, v0,  top, v0, top, top,top }, // join
      new L[]     { bot, bot, top, bot, v0,  v0, bot, bot,v2  }  // meet
    );
  }
}
