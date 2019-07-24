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

import org.chocosolver.solver.*;
import org.chocosolver.solver.variables.*;

// This shows an example of BAB algorithm over the Golomb Ruler problem.
public class SolveBAB
{
  world_line VarStore domains = new VarStore();
  world_line ConstraintStore constraints = new ConstraintStore();
  single_time ES consistent = unknown;

  public proc solve =
    single_space int m = 10;
    init(m, write domains, write constraints);
    single_space IntVar x = rulerLengthVar(m, write domains);
    module MinimizeBAB bab = new MinimizeBAB(write domains, write consistent, write x);
    module Solver solver = new Solver(write domains, write constraints, write consistent);
    par
    <> run solver.searchInputOrderLB()
    <> run bab.solve()
    <> flow when consistent |= true then
             when true |= consistent then printBound(m,domains); end end
       end
    end
  end

  private static void init(int m, VarStore domains, ConstraintStore constraints)
  {
    IntVar[] ticks = new IntVar[m];
    IntVar[] diffs = new IntVar[(m*m -m)/2];
    Model model = domains.model();

    int ub =  (m < 31) ? (1 << (m + 1)) - 1 : 9999;
    for(int i=0; i < ticks.length; i++) {
      ticks[i] = (IntVar) domains.alloc(new VarStore.IntDomain(0, ub, true));
    }
    for(int i=0; i < diffs.length; i++) {
      diffs[i] = (IntVar) domains.alloc(new VarStore.IntDomain(0, ub, true));
    }

    constraints.join_in_place(model.arithm(ticks[0], "=", 0));
    for (int i = 0; i < m - 1; i++) {
      constraints.join_in_place(model.arithm(ticks[i + 1], ">", ticks[i]));
    }

    IntVar[][] m_diffs = new IntVar[m][m];
    for (int k = 0, i = 0; i < m - 1; i++) {
      for (int j = i + 1; j < m; j++, k++) {
        // d[k] is m[j]-m[i] and must be at least sum of first j-i integers
        // <cpru 04/03/12> it is worth adding a constraint instead of a view
        constraints.join_in_place(model.scalar(new IntVar[]{ticks[j], ticks[i]}, new int[]{1, -1}, "=", diffs[k]));
        constraints.join_in_place(model.arithm(diffs[k], ">=", (j - i) * (j - i + 1) / 2));
        constraints.join_in_place(model.arithm(diffs[k], "-", ticks[m - 1], "<=", -((m - 1 - j + i) * (m - j + i)) / 2));
        constraints.join_in_place(model.arithm(diffs[k], "<=", ticks[m - 1], "-", ((m - 1 - j + i) * (m - j + i)) / 2));
        m_diffs[i][j] = diffs[k];
      }
    }
    constraints.join_in_place(model.allDifferent(diffs, "BC"));

    // break symmetries
    if (m > 2) {
      constraints.join_in_place(model.arithm(diffs[0], "<", diffs[diffs.length - 1]));
    }
  }

  private static IntVar rulerLengthVar(int m, VarStore domains) {
    return (IntVar)domains.model().getVars()[m - 1];
  }

  private static int rulerLength(int m, VarStore domains) {
    return rulerLengthVar(m, domains).getLB();
  }

  private static void printBound(int m, VarStore domains) {
    System.out.println("Current bound = " + rulerLength(m, domains));
  }
}

