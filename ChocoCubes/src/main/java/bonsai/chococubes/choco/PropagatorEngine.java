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

import org.chocosolver.solver.*;
import org.chocosolver.solver.exception.*;
import org.chocosolver.util.ESat;

public class PropagatorEngine
{
  public static Consistent propagate(VarStore vstore, ConstraintStore cstore) {
    Solver solver = vstore.model().getSolver();
    try {
      solver.propagate();
    }
    catch (ContradictionException e) {
      solver.getEngine().flush();
    }
    ESat consistency = solver.isSatisfied();
    if (consistency == ESat.TRUE) {
      return Consistent.True;
    }
    else if (consistency == ESat.FALSE) {
      return Consistent.False;
    }
    else {
      return Consistent.Unknown;
    }
  }
}
