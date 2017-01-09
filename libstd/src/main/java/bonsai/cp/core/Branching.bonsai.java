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

package bonsai.cp.core;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import org.chocosolver.solver.*;
import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.search.strategy.selectors.variables.*;
import org.chocosolver.solver.search.strategy.selectors.values.*;
import bonsai.runtime.core.*;
import bonsai.runtime.choco.*;
import bonsai.runtime.sugarcubes.*;

public class Branching implements Executable
{
  private channel world_line VarStore domains = bot;
  private channel world_line ConstraintStore constraints = bot;
  private channel single_time FlatLattice<Consistent> consistent = bot;

  private VariableSelector<IntVar> var;
  private IntValueSelector val;

  public static Branching firstFailMiddle(Model model) {
    return new Branching(new FirstFail(model), new IntDomainMiddle(true));
  }

  public static Branching inputOrderMin(Model model) {
    return new Branching(new InputOrder(model), new IntDomainMin());
  }

  public Branching(VariableSelector<IntVar> var, IntValueSelector val) {
    this.var = var;
    this.val = val;
  }

  public proc execute() {
    split();
  }

  public proc exclude() {
    loop {
      when consistent |= Consistent.Unknown {
        single_time IntVar x = var.getVariable(domains.vars());
        single_time Integer v = val.selectValue(x);
        space
        || constraints <- x.eq(v);
        || constraints <- x.ne(v);
        end
      }
      pause;
    }
  }

  public proc split() {
    loop {
      when consistent |= Consistent.Unknown {
        single_time IntVar x = var.getVariable(domains.vars());
        single_time Integer v = val.selectValue(x);
        space
        || constraints <- x.le(v);
        || constraints <- x.gt(v);
        end
      }
      pause;
    }
  }

}
