// Copyright 2017 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// `Module` is used to control the lifetime of a module by initializing and destroying its fields.
/// It is used immediately after the declaration of the module local var: `LocalVar(new ModuleVar(...), new Module(...))`
/// If the module variable is a field, then it is handled either:
///  (1) By the recursive `__init` calls of the local variable containing the instantiated module.
///  (2) If it is the first module (invoked by the user), then the duplication of the public processes invoke `__init`.

package bonsai.runtime.sugarcubes;

import java.util.function.*;
import bonsai.runtime.core.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class Module extends Scoped
{
  private Consumer<SpaceEnvironment> init;
  private Consumer<SpaceEnvironment> destroy;

  public Module(Consumer<SpaceEnvironment> init,
    Consumer<SpaceEnvironment> destroy, Program body)
  {
    super(body);
    this.init = init;
    this.destroy = destroy;
  }

  public String actualToString() {
    return "<module-init>;\n" + body;
  }

  public Module copy() {
    return new Module(init, destroy, body.copy());
  }

  public Module prepareFor(Environment env) {
    Module copy = new Module(init, destroy, body.prepareFor(env));
    copy.body.setParent(copy);
    return copy;
  }

  protected void enterScope(SpaceEnvironment env) {
    init.accept(env);
  }

  protected void exitScope(SpaceEnvironment env) {
    destroy.accept(env);
  }
}
