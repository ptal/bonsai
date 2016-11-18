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

import java.util.*;
import bonsai.chococubes.core.*;
import org.chocosolver.solver.expression.discrete.relational.*;
import org.chocosolver.solver.constraints.*;
import org.chocosolver.solver.exception.ContradictionException;

public class ConstraintStore extends LatticeVar implements Restorable {

  public ArrayDeque<Constraint> constraints;

  public static ConstraintStore bottom() {
    return new ConstraintStore();
  }

  public ConstraintStore() {
    constraints = new ArrayDeque();
  }

  public Object label() {
    return new Integer(constraints.size());
  }

  // precondition: Restoration strategy of `VarStore` only support depth-first search exploration.
  public void restore(Object label) {
    assert label != null && label.getClass().isInstance(Integer.class) :
      "Label of `ConstraintStore` must be a `Integer` value.";
    Integer newSize = (Integer) label;
    while (constraints.size() != newSize) {
      Constraint c = constraints.pop();
      c.getPropagator(0).getModel().unpost(c);
    }
  }

  public void join(Object value) {
    assert value != null;
    Constraint c;
    if (value instanceof ReExpression) {
      ReExpression expr = (ReExpression) value;
      c = expr.decompose();
    }
    else if (value instanceof Constraint) {
      c = (Constraint) value;
    }
    else {
      throw new RuntimeException(
        "Join in `ConstraintStore` is not defined for `" + value.getClass().getName() +
        "`.\nIt is defined for `Constraint` or relational expression `ReExpression`.");
    }
    constraints.push(c);
    c.post();
  }

  public EntailmentResult entail(Object value) {
    throw new UnsupportedOperationException(
      "Entailment is currently not defined for `ConstraintStore`.");
  }
}
