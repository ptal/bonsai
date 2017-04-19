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

public class Branching implements Executable, Resettable<Branching>
{
  private ref world_line VarStore domains = bot;
  private ref world_line ConstraintStore constraints = bot;
  private ref single_time L<Consistent> consistent = bot;

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

  public Branching() {}

  public void reset(Branching branching) {
    this.var = branching.var;
    this.val = branching.val;
  }

  public proc execute() {
    split();
  }

  public proc exclude() {
    loop {
      when consistent |= Consistent.Unknown {
        single_time L<IntVar> x = new L(var.getVariable(domains.vars()));
        single_time L<Integer> v = new L(val.selectValue(x.unwrap()));
        space
        || constraints <- x.unwrap().eq(v.unwrap());
        || constraints <- x.unwrap().ne(v.unwrap());
        end
      }
      pause;
    }
  }

  public proc split() {
    loop {
      when consistent |= Consistent.Unknown {
        single_time L<IntVar> x2 = new L(var.getVariable(domains.vars()));
        single_time L<Integer> v2 = new L(val.selectValue(x2.unwrap()));
        space
        || constraints <- x2.unwrap().le(v2.unwrap());
        || constraints <- x2.unwrap().gt(v2.unwrap());
        end
      }
      pause;
    }
  }

}
