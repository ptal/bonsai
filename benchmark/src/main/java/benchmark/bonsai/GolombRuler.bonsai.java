package benchmark.bonsai;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import org.chocosolver.solver.*;
import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.constraints.nary.alldifferent.*;
import org.chocosolver.solver.search.strategy.selectors.variables.*;
import org.chocosolver.solver.search.strategy.selectors.values.*;
import bonsai.chococubes.core.*;
import bonsai.chococubes.choco.*;
import bonsai.chococubes.sugarcubes.*;

public class GolombRuler implements Executable
{
  world_line VarStore domains = bot;
  world_line ConstraintStore constraints = bot;
  single_space ConstraintStore objective = bot;
  single_time FlatLattice<Consistent> consistent = bot;

  private static int m = 11;

  proc execute() {
    model();
    engine();
  }

  proc model() {
    ~modelChoco(domains, constraints);
  }

  proc engine() {
    par
    || input_order_lb();
    || propagation();
    || optimize();
    end
  }

  proc input_order_lb() {
    single_space InputOrder var = new InputOrder(domains.model());
    single_space IntDomainMin val = new IntDomainMin();
    loop {
      when consistent |= Consistent.Unknown {
        single_time IntVar x = var.getVariable(domains.vars());
        single_time Integer min = val.selectValue(x);
        space
        || constraints <- x.eq(min);
        || constraints <- x.ne(min);
        end
      }
      pause;
    }
  }

  proc propagation() {
    loop {
      consistent <- PropagatorEngine.propagate(domains, constraints);
      pause;
    }
  }

  proc optimize() {
    loop {
      when consistent |= Consistent.True {
        single_time IntVar obj = rulerLengthVar(domains);
        objective <- obj.lt(rulerLength(domains));
        ~incSolution();
        ~printNumberSolution(rulerLength(domains));
      }
      pause;
    }
  }

  private static void modelChoco(VarStore domains,
    ConstraintStore constraints)
  {
    IntVar[] ticks = new IntVar[m];
    IntVar[] diffs = new IntVar[(m*m -m)/2];
    Model model = domains.model();

    int ub =  (m < 31) ? (1 << (m + 1)) - 1 : 9999;
    for(int i=0; i < ticks.length; i++) {
      ticks[i] = (IntVar) domains.alloc(new IntDomain(0, ub, true));
    }
    for(int i=0; i < diffs.length; i++) {
      diffs[i] = (IntVar) domains.alloc(new IntDomain(0, ub, true));
    }

    constraints.join(model.arithm(ticks[0], "=", 0));
    for (int i = 0; i < m - 1; i++) {
      constraints.join(model.arithm(ticks[i + 1], ">", ticks[i]));
    }

    IntVar[][] m_diffs = new IntVar[m][m];
    for (int k = 0, i = 0; i < m - 1; i++) {
      for (int j = i + 1; j < m; j++, k++) {
        // d[k] is m[j]-m[i] and must be at least sum of first j-i integers
        // <cpru 04/03/12> it is worth adding a constraint instead of a view
        constraints.join(model.scalar(new IntVar[]{ticks[j], ticks[i]}, new int[]{1, -1}, "=", diffs[k]));
        constraints.join(model.arithm(diffs[k], ">=", (j - i) * (j - i + 1) / 2));
        constraints.join(model.arithm(diffs[k], "-", ticks[m - 1], "<=", -((m - 1 - j + i) * (m - j + i)) / 2));
        constraints.join(model.arithm(diffs[k], "<=", ticks[m - 1], "-", ((m - 1 - j + i) * (m - j + i)) / 2));
        m_diffs[i][j] = diffs[k];
      }
    }
    constraints.join(model.allDifferent(diffs, "BC"));

    // break symetries
    if (m > 2) {
      constraints.join(model.arithm(diffs[0], "<", diffs[diffs.length - 1]));
    }
  }

  private static IntVar rulerLengthVar(VarStore domains) {
    return (IntVar)domains.model().getVars()[m - 1];
  }

  private static int rulerLength(VarStore domains) {
    return rulerLengthVar(domains).getLB();
  }

  private static void printHeader(String message,
    FlatLattice<Consistent> consistent)
  {
    System.out.print("["+message+"][" + consistent + "]");
  }

  private static void printModel(String message,
    FlatLattice<Consistent> consistent, VarStore domains)
  {
    printHeader(message, consistent);
    System.out.print(domains.model());
  }

  private static void printVariables(String message,
    FlatLattice<Consistent> consistent, VarStore domains)
  {
    printHeader(message, consistent);
    System.out.print(" Variables = [");
    for (IntVar v : domains.vars()) {
      System.out.print(v + ", ");
    }
    System.out.println("]");
  }

  private static void printNumberSolution(int obj) {
    System.out.println("Number of solutions: " + sol + "[obj = " + obj + "]");
  }

  private static int sol = 0;
  private static void incSolution() {
    sol = sol + 1;
  }
}