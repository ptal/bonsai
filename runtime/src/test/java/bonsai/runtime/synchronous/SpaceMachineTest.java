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
import bonsai.runtime.synchronous.expressions.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.exceptions.*;
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

  public Statement createNestedQFUniverse(Statement code, int remaining) {
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
      Statement process = createNestedQFUniverse(new Nothing(), numLayers);
      SpaceMachine machine = new SpaceMachine(process, true);
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
      Statement process = createNestedQFUniverse(procedure, numLayers);
      SpaceMachine machine = new SpaceMachine(process, true);
      assertTerminated(machine);
      assertThat(currentTest, numCall, equalTo(new LMax(1)));
    }
  }

  @Test(expected = CausalException.class)
  public void testNonCausalProgram() {
    String xUID = "x_uid";
    currentTest = "single_space LMax x = new LMax(1); f(read x, write x);";
    LMax numCall = new LMax(1);
    Consumer<ArrayList<Object>> f = (args) -> fail(currentTest+": function f should not be called");
    List<Access> accesses = Arrays.asList(new ReadAccess(xUID), new WriteAccess(xUID));
    ProcedureCall procedure = new ProcedureCall(accesses, f);
    SingleSpaceVarDecl process = new SingleSpaceVarDecl(xUID,
      new FunctionCall(Arrays.asList(), (args) -> new LMax(1)),
      procedure);
    SpaceMachine machine = new SpaceMachine(process, true);
    machine.execute();
  }
}
