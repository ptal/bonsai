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
  public world_line VarStore domains = new VarStore();
  public world_line ConstraintStore constraints = new ConstraintStore();
  single_space ArrayList<IntVar> queens = new ArrayList();

  public proc nqueens =
    modelNQueens(8, write queens, write domains, write constraints);
    run search_one_sol()
  end

  public proc search_one_sol =
    par
    <> run search()
    <> loop
        when consistent |= true then
          when true |= consistent then
            printSolution(queens);
            stop;
          else pause end
        else pause end
       end
    end
  end

  public proc search = par run propagation() <> run branch() end

  flow propagation =
    consistent <- constraints.propagate(readwrite domains);
    when consistent |= true then prune end
  end

  proc branch =
    single_space VariableSelector<IntVar> var = firstFail(domains);
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

  private VariableSelector<IntVar> firstFail(VarStore domains) {
    return new FirstFail(domains.model());
  }

  private IntValueSelector middle() {
    return new IntDomainMiddle(true);
  }

  // Interface to the Choco solver.
  private IntVar failFirstVar(VariableSelector<IntVar> var, VarStore domains) {
    return var.getVariable(domains.vars());
  }

  private Integer middleValue(IntValueSelector val, IntVar x) {
    return val.selectValue(x);
  }

  private void modelNQueens(int n, ArrayList<IntVar> queens, VarStore domains,
    ConstraintStore constraints)
  {
    IntVar[] vars = new IntVar[n];
    IntVar[] diag1 = new IntVar[n];
    IntVar[] diag2 = new IntVar[n];
    for(int i = 0; i < n; i++) {
      vars[i] = (IntVar) domains.alloc(new VarStore.IntDomain(1, n, false));
      diag1[i] = domains.model().intOffsetView(vars[i], i);
      diag2[i] = domains.model().intOffsetView(vars[i], -i);
    }
    constraints.join_in_place(new AllDifferent(vars, "BC"));
    constraints.join_in_place(new AllDifferent(diag1, "BC"));
    constraints.join_in_place(new AllDifferent(diag2, "BC"));
    for(IntVar v : vars) { queens.add(v); }
  }

  private void printSolution(ArrayList<IntVar> queens) {
    System.out.print("solution: [");
    for (IntVar v : queens) {
      System.out.print(v.getValue() + ",");
    }
    System.out.println("]");
  }

}
