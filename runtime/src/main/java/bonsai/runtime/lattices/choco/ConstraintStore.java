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

package bonsai.runtime.lattices.choco;

import java.util.*;
import bonsai.runtime.core.*;
import org.chocosolver.solver.expression.discrete.relational.*;
import org.chocosolver.solver.constraints.*;
import org.chocosolver.solver.*;
import org.chocosolver.solver.exception.*;
import org.chocosolver.util.ESat;

public class ConstraintStore implements Lattice, Restorable
{
  private ArrayDeque<Constraint> constraints;

  public ConstraintStore bottom() {
    return new ConstraintStore();
  }

  public ConstraintStore() {
    constraints = new ArrayDeque();
  }

  public Integer label() {
    return new Integer(constraints.size());
  }

  // precondition: Restoration strategy of `VarStore` only support depth-first search exploration.
  public void restore(Object label) {
    Cast.checkNull("label", "ConstraintStore.restore", label);
    if (!(label instanceof Integer)) {
      throw new RuntimeException("Label in `ConstraintStore.restore` must be an integer.");
    }
    else {
      Integer newSize = (Integer) label;
      while (constraints.size() != newSize) {
        Constraint c = constraints.pop();
        c.getPropagator(0).getModel().unpost(c);
      }
    }
  }

  public void join_in_place(Object value) {
    Cast.checkNull("argument", "ConstraintStore.join_in_place", value);
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
        "`.\nIt is defined for `Constraint` or the relational expression `ReExpression`.");
    }
    constraints.push(c);
    c.post();
  }

  public ConstraintStore join(Object value) {
    throw new UnsupportedOperationException(
      "`join` is currently not defined for `ConstraintStore`.");
  }

  public void meet_in_place(Object value) {
    throw new UnsupportedOperationException(
      "`meet_in_place` is currently not defined for `ConstraintStore`.");
  }

  public ConstraintStore meet(Object value) {
    throw new UnsupportedOperationException(
      "`meet` is currently not defined for `ConstraintStore`.");
  }

  public Kleene entail(Object value) {
    throw new UnsupportedOperationException(
      "`entail` is currently not defined for `ConstraintStore`.");
  }

  // Spacetime signature: `read this.propagate(readwrite vstore)`
  public Kleene propagate(VarStore vstore) {
    Solver solver = vstore.model().getSolver();
    try {
      solver.propagate();
    }
    catch (ContradictionException e) {
      solver.getEngine().flush();
    }
    switch (solver.isSatisfied()) {
      case TRUE: return Kleene.TRUE;
      case FALSE: return Kleene.FALSE;
      default: return Kleene.UNKNOWN;
    }
  }

  // NOTE: We cannot add a cache to save the consistency between two calls to consistent because the variables might have been changed by another constraint store.
  public Kleene consistent(VarStore vstore) {
    Kleene consistency = Kleene.TRUE;
    for (Constraint c : constraints) {
      switch (consistent(c)) {
        case FALSE: {
          consistency = Kleene.FALSE;
          return consistency;
        }
        case UNKNOWN: {
          consistency = Kleene.UNKNOWN;
          break;
        }
        default: break;
      }
    }
    return consistency;
  }

  private Kleene consistent(Constraint constraint) {
    ESat consistency = constraint.isSatisfied();
    switch (consistency) {
      case TRUE: return Kleene.TRUE;
      case FALSE: return Kleene.FALSE;
      default: return Kleene.UNKNOWN;
    }
  }
}
