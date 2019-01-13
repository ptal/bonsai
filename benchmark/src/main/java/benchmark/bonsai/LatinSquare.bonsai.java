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

package benchmark.bonsai;


import bonsai.cp.Solver;
import bonsai.statistics.Statistics;
import bonsai.statistics.SolutionNode;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;
import bonsai.runtime.core.*;
import bonsai.runtime.lattices.*;
import bonsai.runtime.lattices.choco.*;

import org.chocosolver.solver.*;
import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.constraints.nary.alldifferent.*;

public class LatinSquare
{
  public proc solve() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      world_line VarStore domains = new VarStore();
      world_line ConstraintStore constraints = new ConstraintStore();
      single_time ES consistent = unknown;
      modelChoco(write domains, write constraints, 12);
      module Solver solver = new Solver(domains, constraints, consistent);
      module Statistics stats = new Statistics(consistent);
      module SolutionNode solutions = new SolutionNode(consistent);
      space nothing end; pause;
      par
      <> run solver.solve();
      <> run stats.count();
      <> flow when solutions.value |= 1 then stop end end
      end
    end
  end

  private static void modelChoco(VarStore domains,
    ConstraintStore constraints, int m)
  {
    Model model = domains.model();
    IntVar[] vars = new IntVar[m*m];
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < m; j++) {
        vars[i * m + j] = (IntVar) domains.alloc(new VarStore.IntDomain(0, m - 1));
      }
    }
    // Constraints
    for (int i = 0; i < m; i++) {
      IntVar[] row = new IntVar[m];
      IntVar[] col = new IntVar[m];
      for (int x = 0; x < m; x++) {
        row[x] = vars[i * m + x];
        col[x] = vars[x * m + i];
      }
      constraints.join_in_place(model.allDifferent(col, "AC"));
      constraints.join_in_place(model.allDifferent(row, "AC"));
    }
  }
}

