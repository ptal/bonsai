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
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.statements.*;
import bonsai.runtime.synchronous.interfaces.*;
import static org.junit.Assert.*;
import static org.hamcrest.CoreMatchers.*;
import org.junit.*;

public class SpaceMachineTest
{
  protected String currentTest;

  public void assertTerminated(SpaceMachine machine) {
    // Check for idempotency once terminated.
    for(int i=0; i < 3; i++) {
      boolean state = machine.execute();
      assertThat(currentTest, state, equalTo(false));
    }
  }

  public Program createNestedQFUniverse(int remaining) {
    if (remaining == 0) {
      return new Nothing();
    }
    else {
      return new QFUniverse(createNestedQFUniverse(remaining-1));
    }
  }

  @Test
  public void testNothingQFUniverse() {
    for(int numLayers=1; numLayers < 4; numLayers++) {
      currentTest = "universe^"+numLayers+" nothing end";
      Program process = createNestedQFUniverse(numLayers);
      SpaceMachine machine = new SpaceMachine(process, numLayers);
      assertTerminated(machine);
    }
  }
}
