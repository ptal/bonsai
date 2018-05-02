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

package bonsai.runtime.sugarcubes;

import java.util.function.*;
import bonsai.runtime.core.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public abstract class Scoped extends UnaryInstruction
{
  protected boolean firstActivation;

  public Scoped(Program body)
  {
    super(body);
    this.firstActivation = true;
  }

  public byte activate(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    if (firstActivation) {
      firstActivation = false;
      enterScope(env);
    }
    byte res = body.activate(env);
    lastActivation(env, res);
    return res;
  }

  protected abstract void enterScope(SpaceEnvironment env);
  protected abstract void exitScope(SpaceEnvironment env);

  /// Check if the variable exits its scope.
  private void lastActivation(SpaceEnvironment env, byte res) {
    if (TERM == res || EXCP == res) {
      exitScope(env);
    }
  }

  public void notifyTermination(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    if (!firstActivation) {
      firstActivation = true;
      exitScope(env);
    }
    body.notifyTermination(env);
  }
}
