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

package bonsai.chococubes.choco;

import bonsai.chococubes.core.*;
import org.chocosolver.solver.expression.discrete.relational.*;

public class ConstraintStore extends LatticeVar implements Restorable {

  public static ConstraintStore bottom() {
    return new ConstraintStore();
  }

  public Object label() {
    throw new UnsupportedOperationException();
  }

  public void restore(Object label) {
    throw new UnsupportedOperationException();
  }

  public void join(Object value) {
    assert value != null && value.getClass().isInstance(ReExpression.class) :
      "Join in `ConstraintStore` is only defined for relational expression `ReExpression`.";
    ReExpression constraint = (ReExpression) value;
    constraint.post();
  }

  public EntailmentResult entail(Object value) {
    assert false :
      "Entailment is currently not defined for `ConstraintStore`.";
    throw new UnsupportedOperationException();
  }
}
