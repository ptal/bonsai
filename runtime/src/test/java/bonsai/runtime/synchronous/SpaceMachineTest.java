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

  public Program createNestedQFUniverse(Program code, int remaining) {
    if (remaining == 0) {
      return code;
    }
    else {
      return createNestedQFUniverse(new QFUniverse(code), remaining-1);
    }
  }

  @Test
  public void testNothingQFUniverse() {
    for(int numLayers=0; numLayers < 4; numLayers++) {
      currentTest = "universe^"+numLayers+" nothing end";
      Program process = createNestedQFUniverse(new Nothing(), numLayers);
      SpaceMachine machine = new SpaceMachine(process, numLayers, true);
      assertTerminated(machine);
    }
  }

  @Test
  public void testProcedureQFUniverse() {
    for(int numLayers=0; numLayers < 4; numLayers++) {
      currentTest = "universe^"+numLayers+" f() end";
      LMax numCall = new LMax(0);
      Consumer<ArrayList<Object>> f = (args) -> numCall.inc();
      ProcedureCall procedure = new ProcedureCall(new ArrayList(), f);
      Program process = createNestedQFUniverse(procedure, numLayers);
      SpaceMachine machine = new SpaceMachine(process, numLayers, true);
      assertTerminated(machine);
      assertThat(currentTest, numCall, equalTo(new LMax(1)));
    }
  }

  // @Test throw CausalException
  // public void testNonCausalProgram() {
  //   currentTest = "universe f(read x, write x) end";
  //   LMax numCall = new LMax(0);
  //   Consumer<ArrayList<Object>> f = (args) -> fail(currentTest+": function f should not be called");
  //   List<Access> args = List.of(new ReadAccess("x"), new WriteAccess("x"));
  //   ProcedureCall procedure = new ProcedureCall(args, f);
  //   Program process = createNestedQFUniverse(procedure, numLayers);
  //   SpaceMachine machine = new SpaceMachine(process, numLayers, true);
  //   assertTerminated(machine);
  //   assertThat(currentTest, numCall, equalTo(new LMax(1)));
  // }
}
