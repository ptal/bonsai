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

package bonsai.solver;

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

public class Solver
{
  ref world_line VarStore domains;
  ref world_line ConstraintStore constraints;
  ref single_time ES consistent;

  public Solver(VarStore domains, ConstraintStore constraints, ES consistent) {
    this.domains = domains;
    this.constraints = constraints;
    this.consistent = consistent;
  }

  public proc solve() =
    par
    || run failFirstMiddle()
    || run propagation()
    end
  end

  public flow propagation() =
    consistent <- constraints.propagate(readwrite domains);
    when consistent |= true then prune end
  end

  public proc failFirstMiddle() =
    single_space VariableSelector<IntVar> var = new FirstFail(domains.model());
    single_space IntValueSelector val = createValueSelector();
    flow
      when unknown |= consistent then
        single_time IntVar x = readwrite var.getVariable(domains.vars());
        single_time Integer v = readwrite val.selectValue(x);
        space constraints <- x.eq(v) end;
        space constraints <- x.ne(v) end
      end
    end
  end

  private static IntValueSelector createValueSelector() {
    return new IntDomainMiddle​(true);
  }
}
