package bonsai.examples;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.constraints.nary.alldifferent.*;
import org.chocosolver.solver.search.strategy.selectors.variables.*;
import org.chocosolver.solver.search.strategy.selectors.values.*;
import bonsai.runtime.core.*;
import bonsai.runtime.choco.*;
import bonsai.runtime.sugarcubes.*;
import bonsai.cp.core.*;

public class NQueens
{
  private world_line VarStore domains = bot;
  private world_line ConstraintStore constraints = bot;
  private single_time L<Consistent> consistent = bot;

  private int n;

  public NQueens(int n) {
    this.n = n;
  }

  public NQueens() {
    this.n = 8;
  }

  public proc execute() {
    model();
    trap FoundSolution {
      engine();
    }
    ~printVariables("Solution", consistent, domains);
  }

  proc model() {
    ~modelChoco(domains, constraints);
    ~printModel("After initialization", consistent, domains);
  }

  proc engine() {
    module Branching branching = Branching.firstFailMiddle(domains.model());
    module Propagation propagation = new Propagation();
    // module OneSolution oneSolution = new OneSolution();
    par
    || run branching.split();
    || run propagation;
    || loop {
        par
        || when domains |= constraints
           {
            ~printVariables("Solution", consistent, domains);
            stop;
           }
        || pause;
        end
       }
    end
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
    L<Consistent> consistent)
  {
    System.out.print("["+message+"][" + consistent + "]");
  }

  private static void printModel(String message,
    L<Consistent> consistent, VarStore domains)
  {
    printHeader(message, consistent);
    System.out.print(domains.model());
  }

  private static void printVariables(String message,
    L<Consistent> consistent, VarStore domains)
  {
    printHeader(message, consistent);
    System.out.print(" Variables = [");
    for (IntVar v : domains.vars()) {
      System.out.print(v + ", ");
    }
    System.out.println("]");
  }
}