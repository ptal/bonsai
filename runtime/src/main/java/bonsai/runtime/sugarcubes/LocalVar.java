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

public class LocalVar extends Scoped
{
  private Variable var;

  public LocalVar(Variable var, Program body)
  {
    super(body);
    this.var = var;
  }

  public String actualToString() {
    return var.name() + " = " + var.value(0) + ";\n" + body;
  }

  public LocalVar copy() {
    return new LocalVar(var, body.copy());
  }

  public LocalVar prepareFor(Environment env) {
    LocalVar copy = new LocalVar(var, body.prepareFor(env));
    copy.body.setParent(copy);
    return copy;
  }

  protected void enterScope(SpaceEnvironment env) {
    var.reset(env);
    env.enterScope(var);
  }

  protected void exitScope(SpaceEnvironment env) {
    env.exitScope(var.uid());
  }
}
