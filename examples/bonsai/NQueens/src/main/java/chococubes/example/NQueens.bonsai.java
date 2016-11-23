package chococubes.example;

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

  private int n;

  public NQueens(int n) {
    this.n = n;
  }

  public NQueens() {
    this.n = 8;
  }

  proc execute() {
    model();
    engine();
    ~printVariables("Solution", consistent, domains);
  }

  proc model() {
    ~modelChoco(domains, constraints);
    ~printModel("After initialization", consistent, domains);
  }

  proc engine() {
    trap FoundSolution {
      par
      || first_fail_middle();
      || propagation();
      || one_solution();
      end
    }
  }

  proc first_fail_middle() {
    single_space FirstFail var = new FirstFail(domains.model());
    single_space IntDomainMiddle val = new IntDomainMiddle(IntDomainMiddle.FLOOR);
    loop {
      when consistent |= Consistent.Unknown {
        single_time IntVar x = var.getVariable(domains.vars());
        single_time Integer mid = val.selectValue(x);
        space
        || constraints <- x.le(mid);
        || constraints <- x.gt(mid);
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

  proc one_solution() {
    loop {
      when consistent |= Consistent.True {
        exit FoundSolution;
      }
      pause;
    }
  }

  private void modelChoco(VarStore domains,
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
}