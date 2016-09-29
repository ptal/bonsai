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

package bonsai.chococubes.sugarcubes;

import bonsai.chococubes.core.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class SpaceBranch extends UnaryInstruction
{
  public SpaceBranch(Program body) {
    super(body);
  }

  public String actualToString() {
    return super.body.toString();
  }

  public Instruction copy() {
    return new SpaceBranch(body.copy());
  }

  // Do not copy the branch here because it is kept in SpaceEnvironment.
  public Instruction prepareFor(Environment env) {
    body = body.prepareFor(env);
    body.setParent(this);
    return this;
  }

  public byte activate(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    env.enterSpaceBranch();
    byte res = super.activate(env);
    env.exitSpaceBranch();
    return res;
  }
}
