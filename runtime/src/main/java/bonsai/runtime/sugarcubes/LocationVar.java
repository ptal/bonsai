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

public class LocationVar extends SpacetimeVar
{
  private String storeName;

  public LocationVar(String name, String storeName,
    Function<SpaceEnvironment, Object> initValue, Program body)
  {
    super(name, false, Spacetime.SingleSpace, false, 1,
      (env) -> {
        Store store = (Store) env.var(storeName, 0);
        Object value = initValue.apply(env);
        return store.alloc(value);
      },
      body);
    this.storeName = storeName;
  }

  public LocationVar(LocationVar var) {
    super(var.name, var.isModuleAttr, var.spacetime,
      var.initValue, var.stream, var.body.copy());
    this.storeName = var.storeName;
  }

  public String actualToString() {
    return name + " = " + storeName + " <- <expression>;\n" + body;
  }

  public Instruction copy() {
    return new LocationVar(this);
  }

  public Instruction prepareFor(Environment env) {
    LocationVar copy = new LocationVar(this);
    copy.body = copy.body.prepareFor(env);
    copy.body.setParent(copy);
    return copy;
  }
}
