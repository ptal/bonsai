package bonsai.cp.core;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import org.chocosolver.solver.*;
import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.search.strategy.selectors.variables.*;
import org.chocosolver.solver.search.strategy.selectors.values.*;
import bonsai.chococubes.core.*;
import bonsai.chococubes.choco.*;
import bonsai.chococubes.sugarcubes.*;

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
