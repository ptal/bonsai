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
import inria.meije.rc.sugarcubes.implementation.*;
import inria.meije.rc.sugarcubes.*;

// `v <- e` is converted into a closure of type `SideEffect` modifying the variable `v` with the expression `e`.
// It must be checked statically before that `e` do not depends on variable that could be modified after this operation. This would lead to non-determinism, take for example `v <- x || x <- [0..1]`.

public class Tell extends Emit
{
  private String leftSide;
  private SideEffect rightSide;

  public Tell(ClockIdentifier clockID, String leftSide, SideEffect rightSide) {
    super(clockID, new StringID(leftSide), null, true);
    this.leftSide = leftSide;
    this.rightSide = rightSide;
  }

  public byte activate(Environment env) {
    SpaceEnvironment space_env = (SpaceEnvironment) env;
    LatticeVar lhs = space_env.var(leftSide);
    Object rhs = rightSide.apply(space_env);
    lhs.join(rhs);
    return super.activate(space_env);
  }
}
