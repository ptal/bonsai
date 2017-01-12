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
import inria.meije.rc.sugarcubes.implementation.*;
import inria.meije.rc.sugarcubes.*;

// The right part of `v <- e` is converted into a closure of type `Function<SpaceEnvironment, Object>` returning a value based on the current environment.
// It must be checked statically before that `e` do not depends on variable that could be modified after this operation. This would lead to non-determinism, take for example `v <- x || x <- [0..1]`.

public class Tell extends PureGenerateIdentifier
{
  private String leftSide;
  private Function<SpaceEnvironment, Object> rightSide;

  public Tell(String leftSide, Function<SpaceEnvironment, Object> rightSide) {
    super(new StringID(leftSide));
    this.leftSide = leftSide;
    this.rightSide = rightSide;
  }

  public String actualToString(){
    return leftSide + " <- <expression>;\n";
  }

  public Instruction copy(){
    return new Tell(leftSide, rightSide);
  }

  public Instruction prepareFor(final Environment env){
    Tell copy = (Tell) this.copy();
    copy.event = env.getDirectAccessToEvent(eventIdentifier);
    return copy;
  }

  public boolean action(Environment e){
    SpaceEnvironment env = (SpaceEnvironment) e;
    LatticeVar lhs = env.latticeVar(leftSide, 0);
    Object rhs = rightSide.apply(env);
    lhs.join(rhs);
    return super.action(env);
  }
}
