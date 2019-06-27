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
  public ref single_time ES consistent;
  public ref world_line VarStore domains;
  public ref world_line ConstraintStore constraints;

  public Solver(VarStore domains, ConstraintStore constraints, ES consistent) {
    this.domains = domains;
    this.constraints = constraints;
    this.consistent = consistent;
  }

  public proc search = par run propagation() <> run branch() end

  flow propagation =
    consistent <- constraints.propagate(readwrite domains);
    when consistent |= true then prune end
  end

  proc branch =
    single_space VariableSelector<IntVar> var = firstFail(write domains);
    single_space IntValueSelector val = middle();
    flow
      when unknown |= consistent then
        single_time IntVar x = failFirstVar(var, domains);
        single_time Integer v = middleValue(val, x);
        space constraints <- x.le(v) end;
        space constraints <- x.gt(v) end
      end
    end
  end

  // This strategy is to illustrate Golomb Ruler with BAB.
  public proc searchInputOrderLB = par run propagation() <> run branchInputOrderLB() end

  proc branchInputOrderLB =
    single_space VariableSelector<IntVar> var = inputOrder(write domains);
    single_space IntValueSelector val = min();
    flow
      when unknown |= consistent then
        single_time IntVar x = readwrite var.getVariable(domains.vars());
        single_time Integer v = readwrite val.selectValue(x);
        space readwrite domains.join_eq(x, v) end;
        space readwrite domains.join_neq(x, v) end
      end
    end
  end

  private VariableSelector<IntVar> firstFail(VarStore domains) {
    return new FirstFail(domains.model());
  }

  private IntValueSelector middle() {
    return new IntDomainMiddle(true);
  }

  private VariableSelector<IntVar> inputOrder(VarStore domains) {
    return new InputOrder(domains.model());
  }

  private IntValueSelector min() {
    return new IntDomainMin();
  }

  // Interface to the Choco solver.
  private IntVar failFirstVar(VariableSelector<IntVar> var, VarStore domains) {
    return var.getVariable(domains.vars());
  }

  private Integer middleValue(IntValueSelector val, IntVar x) {
    return val.selectValue(x);
  }
}
