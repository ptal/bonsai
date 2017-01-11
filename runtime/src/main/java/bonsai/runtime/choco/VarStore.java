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

public class VarStore extends Store implements Restorable {
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

  public void join(Object value) {
    assert value != null;
    if (value instanceof Entry) {
      throw new UnsupportedOperationException(
        "Join is currently not defined for `VarStore` because `IntVar` does not provide intersection.");
    }
    else {
      throw new RuntimeException(
        "Join is only defined between `VarStore` and an entry `VarStore.Entry`.");
    }
  }

  public EntailmentResult entail(Object value) {
    throw new UnsupportedOperationException(
      "Entailment is currently not defined for `VarStore`.");
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
