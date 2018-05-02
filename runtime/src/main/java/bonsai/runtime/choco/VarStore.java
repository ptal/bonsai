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

package bonsai.runtime.choco;

import bonsai.runtime.core.*;
import org.chocosolver.solver.*;
import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.constraints.*;
import org.chocosolver.util.ESat;

public class VarStore implements Store, Restorable {
  private Model model;
  private Integer depth;

  public VarStore bottom() {
    return new VarStore();
  }

  public VarStore() {
    this("Bonsai problem");
  }

  public VarStore(String problem_name) {
    model = new Model(problem_name);
    depth = 0;
  }

  public IntVar[] vars() {
    return model.retrieveIntVars(true);
  }

  public Model model() {
    return model;
  }

  public Object label() {
    return new Integer(depth);
  }

  // precondition: Restoration strategy of `VarStore` only support depth-first search exploration.
  public void restore(Object label) {
    assert label != null;
    if (label instanceof Integer) {
      Integer newDepth = (Integer) label;
      model.getEnvironment().worldPopUntil(newDepth);
      model.getEnvironment().worldPush();
      depth = newDepth + 1;
    }
    else {
      throw new RuntimeException(
        "Label of `VarStore` must be a `Integer` value.");
    }

  }

  public Object alloc(Object value) {
    assert value != null;
    if (value instanceof IntDomain) {
      IntDomain dom = (IntDomain) value;
      return model.intVar(dom.lb, dom.ub, dom.bounded);
    }
    else {
      throw new RuntimeException(
        "Allocation in `VarStore` is only defined for integer domain `IntDomain`.");
    }
  }

  public Object index(Object location) {
    assert location != null;
    if (location instanceof IntVar) {
      // Note that the object in the store is actually the same as the location.
      return location;
    }
    else {
      throw new RuntimeException(
        "Location of `VarStore` must be of type `IntVar`");
    }
  }

  public void join_in_place(Object value) {
    assert value != null;
    if (value instanceof Entry) {
      throw new UnsupportedOperationException(
        "Join is currently not defined for `VarStore` because `IntVar` does not provide intersection.");
    }
    else if (this != value) {
      throw new RuntimeException(
        "Join is only defined between `VarStore` and an entry `VarStore.Entry`.");
    }
  }

  public VarStore join(Object value) {
    throw new UnsupportedOperationException(
      "`join` is currently not defined for `VarStore`.");
  }

  public VarStore meet(Object value) {
    throw new UnsupportedOperationException(
      "`meet` is currently not defined for `VarStore`.");
  }

  public void meet_in_place(Object value) {
    throw new UnsupportedOperationException(
      "`meet_in_place` is currently not defined for `VarStore`.");
  }

  public LMax countAsn() {
    checkOnlyIntVar(model);
    LMax asn = new LMax();
    for (IntVar v : model.retrieveIntVars(true)) {
      if (v.isInstantiated()) {
        asn.inc();
      }
    }
    return asn;
  }

  public LMax countAsnOf(IntVar[] vars) {
    checkOnlyIntVar(model);
    LMax asn = new LMax();
    for (IntVar v : vars) {
      if (v.isInstantiated()) {
        asn.inc();
      }
    }
    return asn;
  }


  public Kleene entail(Object value) {
    if (value instanceof VarStore) {
      VarStore vstore = (VarStore) value;
      return vstoreEntail(vstore);
    }
    else if (value instanceof ConstraintStore) {
      ConstraintStore cstore = (ConstraintStore) value;
      return cstoreEntail(cstore);
    }
    else if (value instanceof Constraint) {
      Constraint constraint = (Constraint) value;
      return constraintEntail(constraint);
    }
    else {
      throw new UnsupportedOperationException(
        "Entailment is not defined between `VarStore` and "
        + value.getClass().getName() + ".");
    }
  }

  private Kleene vstoreEntail(VarStore vstore) {
    checkOnlyIntVar(model);
    checkOnlyIntVar(vstore.model);
    IntVar[] v1 = model.retrieveIntVars(true);
    IntVar[] v2 = vstore.model.retrieveIntVars(true);
    if (v2.length < v1.length) {
      return Kleene.FALSE;
    }
    else if (v1.length < v2.length) {
      return Kleene.UNKNOWN;
    }
    else {
      Kleene res = Kleene.TRUE;
      for (int i = 0; i < v1.length; i++) {
        Kleene varRes = varEntail(v1[i], v2[i]);
        if (varRes == Kleene.FALSE) {
          return Kleene.FALSE;
        }
        else if (varRes == Kleene.UNKNOWN) {
          res = Kleene.UNKNOWN;
        }
      }
      return res;
    }
  }

  private Kleene varEntail(IntVar v1, IntVar v2) {
    if (v1.isInstantiated() && v2.isInstantiated()) {
      if (v1.getValue() == v2.getValue()) {
        return Kleene.TRUE;
      }
      else {
        return Kleene.FALSE;
      }
    }
    else {
      // We could be more precise using the set inclusion between v1 and v2.
      return Kleene.UNKNOWN;
    }
  }

  private Kleene cstoreEntail(ConstraintStore cstore) {
    Kleene res = Kleene.TRUE;
    for (Constraint c : cstore.constraints) {
      Kleene cRes = constraintEntail(c);
      if (cRes == Kleene.FALSE) {
        return Kleene.FALSE;
      }
      else if (cRes == Kleene.UNKNOWN) {
        res = Kleene.UNKNOWN;
      }
    }
    return res;
  }

  private Kleene constraintEntail(Constraint constraint) {
    ESat consistency = constraint.isSatisfied();
    if (consistency == ESat.TRUE) {
      return Kleene.TRUE;
    }
    else if (consistency == ESat.FALSE) {
      return Kleene.FALSE;
    }
    else {
      return Kleene.UNKNOWN;
    }
  }

  private void checkOnlyIntVar(Model model) {
    if (model.retrieveSetVars().length > 0 ||
        model.retrieveRealVars().length > 0)
    {
      throw new RuntimeException(
        "Entailment between two VarStore only works with integer variables.");
    }
  }

  public class Entry {
    private IntVar location;
    private IntDomain value;

    public Entry(IntVar location, IntDomain value) {
      this.location = location;
      this.value = value;
    }
  }
}
