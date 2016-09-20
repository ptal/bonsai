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
import org.chocosolver.solver.variables.*;

public class VarStore {
  Model model;

  static public VarStore bottom() {
    return new VarStore();
  }

  VarStore() {
    this("ChocoCubes problem");
  }

  VarStore(String problem_name) {
    model = new Model(problem_name);
  }

  public IntVar[] vars() {
    return model.retrieveIntVars(true);
  }

  public Model model() {
    return model;
  }

  public IntVar tell_store(IntDomain dom) {
    return model.intVar(dom.lb, dom.ub);
  }
}
