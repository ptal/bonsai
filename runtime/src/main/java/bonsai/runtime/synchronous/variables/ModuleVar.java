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

package bonsai.runtime.synchronous.variables;

import java.util.function.*;
import bonsai.runtime.synchronous.*;

public class ModuleVar extends Variable
{
  private Object ref;
  private Function<Environment, Object> initValue;

  public ModuleVar(String name,
    Function<Environment, Object> initValue)
  {
    super(name);
    this.ref = null;
    this.initValue = initValue;
  }

  public Object value() {
    return ref;
  }

  public void reset(Environment env) {
    ref = initValue.apply(env);
  }
}
