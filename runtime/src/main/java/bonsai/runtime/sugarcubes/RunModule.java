// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.runtime.sugarcubes;

import java.util.function.*;
import bonsai.runtime.core.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class RunModule extends UnaryInstruction
{
  private Function<SpaceEnvironment, Program> invokeModule;
  private boolean firstActivation;

  public RunModule(Function<SpaceEnvironment, Program> invokeModule)
  {
    super();
    this.invokeModule = invokeModule;
    this.firstActivation = true;
  }

  public String actualToString() {
    return "run " + invokeModule;
  }

  public Instruction copy() {
    return new RunModule(invokeModule);
  }

  public Instruction prepareFor(Environment e) {
    RunModule copy = new RunModule(invokeModule);
    copy.body.setParent(copy);
    return copy;
  }

  public byte activate(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    firstActivation(env);
    byte res = body.activate(env);
    return res;
  }

  public void firstActivation(SpaceEnvironment env) {
    if (firstActivation) {
      firstActivation = false;
      body = (Instruction) invokeModule.apply(env);
      body = body.prepareFor(env);
      body.setParent(this);
    }
  }
}
