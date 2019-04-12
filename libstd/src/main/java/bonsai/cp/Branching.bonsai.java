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

package bonsai.cp;

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

public class Branching
{
  ref world_line VarStore domains;
  ref world_line ConstraintStore constraints;
  ref single_time ES consistent;
  ref single_space VariableSelector<IntVar> var;
  ref single_space IntValueSelector val;

  public Branching(VarStore domains, ConstraintStore constraints, ES consistent,
    VariableSelector<IntVar> var, IntValueSelector val)
  {
    this.domains = domains;
    this.constraints = constraints;
    this.consistent = consistent;
    this.var = var;
    this.val = val;
  }

  public flow assign() =
    when unknown |= consistent then
      single_time IntVar x = readwrite var.getVariable(domains.vars());
      single_time Integer v = readwrite val.selectValue(x);
      space readwrite domains.join_eq(x, v) end;
      space readwrite domains.join_neq(x, v) end
    end

  public flow split() =
    when unknown |= consistent then
      single_time IntVar x = readwrite var.getVariable(domains.vars());
      single_time Integer v = readwrite val.selectValue(x);
      space readwrite domains.join_le(x, v) end;
      space readwrite domains.join_gt(x, v) end
    end

  public static VariableSelector<IntVar> inputOrder(VarStore domains) {
    return new InputOrder(domains.model());
  }

  public static VariableSelector<IntVar> firstFail(VarStore domains) {
    return new FirstFail(domains.model());
  }

  public static VariableSelector<IntVar> smallest() {
    return new Smallest();
  }

  public static IntValueSelector min() {
    return new IntDomainMin();
  }

  public static IntValueSelector median() {
    return new IntDomainMedian();
  }

  public static IntValueSelector middle() {
    return new IntDomainMiddle(true);
  }
}
