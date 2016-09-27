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

// This class implements the entailment relation `a |= b` where `a` is always a variable and `b` is an expression (implemented by a lambda expression).
// We only react to `a` modifications, the variables in `b` are supposed constant. We know some modifications on `a` happened if the event of the same name is present.

public class EntailmentConfig extends ConfigImpl implements Precursor
{
  private String leftSide;
  private SideEffect rightSide;
  private Event event;
  private Precursor parent;
  private boolean posted;

  public EntailmentConfig(String leftSide, SideEffect rightSide) {
    this.leftSide = leftSide;
    this.rightSide = rightSide;
    this.posted = false;
  }

  public String toString() {
    return leftSide.toString() + " |= <expression>";
  }

  public Config copy() {
    return new EntailmentConfig(leftSide, rightSide);
  }

  public Config prepareFor(Environment env) {
    EntailmentConfig copy = (EntailmentConfig)copy();
    copy.event = env.getDirectAccessToEvent(new StringID(leftSide));
    return copy;
  }

  public void setPrecursor(Precursor p, byte waikUpOn) {
    this.parent = p;
  }

  public byte evaluate(Environment env) {
    // Change happened on the domain of the variable `leftSide`.
    if(event.isGenerated(env)){
      SpaceEnvironment space_env = (SpaceEnvironment) env;
      LatticeVar lhs = space_env.var(leftSide);
      Object rhs = rightSide.apply(space_env);
      EntailmentResult res = lhs.entail(rhs);
      if (res == EntailmentResult.TRUE) {
        return SATISFIED;
      }
      else if (res == EntailmentResult.FALSE) {
        return UNSATISFIED;
      }
    }
    if(!posted){
      event.postPrecursor(this);
      posted = true;
    }
    return UNKNOWN;
  }

  public byte evaluateAtEOI(Environment env) {
    return this.evaluate(env);
  }

  public void reset() {
  }

  public void zapFromHere(Environment env) {
    parent.zapFromHere(env);
    posted = false;
  }
}
