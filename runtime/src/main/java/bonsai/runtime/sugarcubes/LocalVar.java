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

public class LocalVar extends UnaryInstruction
{
  private SpacetimeVar var;
  private boolean firstActivation;

  public LocalVar(SpacetimeVar var, Program body)
  {
    super(body);
    this.var = var;
    this.firstActivation = true;
  }

  public String actualToString() {
    return var.name() + " in " + var.spacetime() + " = " + var.value(0) + ";\n" + body;
  }

  public LocalVar copy() {
    return new LocalVar(var, body.copy());
  }

  public LocalVar prepareFor(Environment env) {
    LocalVar copy = new LocalVar(var, body.prepareFor(env));
    copy.body.setParent(copy);
    return copy;
  }

  public byte activate(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    enterScope(env);
    byte res = body.activate(env);
    lastActivation(env, res);
    return res;
  }

  public void enterScope(SpaceEnvironment env) {
    if (firstActivation) {
      firstActivation = false;
      var.reset(env);
      env.enterScope(var);
    }
  }

  public void exitScope(SpaceEnvironment env) {
    if (!firstActivation) {
      firstActivation = true;
      env.exitScope(var);
    }
  }

  /// Check if the variable exits its scope.
  public void lastActivation(SpaceEnvironment env, byte res) {
    if (TERM == res || EXCP == res) {
      exitScope(env);
    }
  }

  public void notifyTermination(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    exitScope(env);
    body.notifyTermination(env);
  }
}
