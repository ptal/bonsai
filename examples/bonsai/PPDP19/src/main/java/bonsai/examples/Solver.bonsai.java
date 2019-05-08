// Copyright 2019 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.examples;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;
import bonsai.runtime.core.*;
import bonsai.runtime.lattices.*;
import bonsai.runtime.lattices.choco.*;

import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.constraints.nary.alldifferent.*;
import org.chocosolver.solver.search.strategy.selectors.variables.*;
import org.chocosolver.solver.search.strategy.selectors.values.*;
import org.chocosolver.solver.*;

public class Solver
{
  single_time ES consistent = unknown;
  ref world_line VarStore domains;
  ref world_line ConstraintStore constraints;

  public Solver(VarStore domains, ConstraintStore constraints) {
    this.domains = domains;
    this.constraints = constraints;
  }

  public proc search = par run propagation() <> run branch() end

  flow propagation =
    consistent <- constraints.propagate(readwrite domains);
    when consistent |= true then prune end
  end

  flow branch =
    when unknown |= consistent then
      single_time IntVar x = failFirstVar(domains);
      single_time Integer v = middleValue(x);
      space constraints <- x.le(v) end;
      space constraints <- x.gt(v) end
    end

  // Interface to the Choco solver.
  private IntVar failFirstVar(VarStore domains) {
    VariableSelector<IntVar> var = new FirstFail(domains.model());
    return var.getVariable(domains.vars());
  }

  private Integer middleValue(IntVar x) {
    IntValueSelector val = new IntDomainMiddle(true);
    return val.selectValue(x);
  }
}
