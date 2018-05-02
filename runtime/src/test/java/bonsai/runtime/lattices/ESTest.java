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

public class ESTest
{
  ES t;
  ES f;
  ES u;

  @Before
  public void initES() {
    t = new ES(Kleene.TRUE);
    f = new ES(Kleene.FALSE);
    u = new ES(Kleene.UNKNOWN);
  }

  @Test
  public void testLattice() {
    LatticeTest test = new LatticeTest();

    Kleene T = Kleene.TRUE;
    Kleene F = Kleene.FALSE;
    Kleene U = Kleene.UNKNOWN;

    test.testLattice("ES",
      new ES[]     { t, f, t, f, u, t, u, f, u },
      new ES[]     { t, t, f, f, t, u, u, u, f },
      new Kleene[] { T, T, F, T, F, T, T, T, F }, // entailment
      new ES[]     { t, f, f, f, t, t, u, f, f }, // join
      new ES[]     { t, t, t, f, u, u, u, u, u }  // meet
    );
  }
}
