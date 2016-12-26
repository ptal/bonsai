package benchmark.bonsai;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.constraints.nary.alldifferent.*;
import org.chocosolver.solver.search.strategy.selectors.variables.*;
import org.chocosolver.solver.search.strategy.selectors.values.*;
import bonsai.chococubes.core.*;
import bonsai.chococubes.choco.*;
import bonsai.chococubes.sugarcubes.*;

public class NQueens implements Executable
{
  world_line VarStore domains = bot;
  world_line ConstraintStore constraints = bot;
  single_time FlatLattice<Consistent> consistent = bot;

  private static int n = 14;

  proc execute() {
    model();
    engine();
  }

  proc model() {
    ~modelChoco(domains, constraints);
  }

  proc engine() {
    par
    || first_fail_middle();
    || propagation();
    || all_solution();
    end
  }

  proc first_fail_middle() {
    single_space FirstFail var = new FirstFail(domains.model());
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

  proc all_solution() {
    loop {
      when consistent |= Consistent.True {
        ~incSolution();
        ~printNumberSolution();
      }
      pause;
    }
  }

  private static void modelChoco(VarStore domains,
    ConstraintStore constraints)
  {
    IntVar[] vars = new IntVar[n];
    IntVar[] diag1 = new IntVar[n];
    IntVar[] diag2 = new IntVar[n];
    for(int i = 0; i < n; i++) {
      vars[i] = (IntVar) domains.alloc(new IntDomain(1, n));
      diag1[i] = domains.model().intOffsetView(vars[i], i);
      diag2[i] = domains.model().intOffsetView(vars[i], -i);
    }
    constraints.join(new AllDifferent(vars, "BC"));
    constraints.join(new AllDifferent(diag1, "BC"));
    constraints.join(new AllDifferent(diag2, "BC"));
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

  private static void printNumberSolution() {
    System.out.println("Number of solutions: " + sol);
  }

  private static int sol = 0;
  private static void incSolution() {
    sol = sol + 1;
  }
}