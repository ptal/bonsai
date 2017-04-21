package benchmark.bonsai;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import org.chocosolver.solver.*;
import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.constraints.nary.alldifferent.*;
import org.chocosolver.solver.search.strategy.selectors.variables.*;
import org.chocosolver.solver.search.strategy.selectors.values.*;
import bonsai.runtime.core.*;
import bonsai.runtime.choco.*;
import bonsai.runtime.sugarcubes.*;

public class LatinSquare
{
  world_line VarStore domains = bot;
  world_line ConstraintStore constraints = bot;
  single_time L<Consistent> consistent = bot;

  private static int m = 60;

  public proc execute() {
    model();
    engine();
  }

  proc model() {
    ~modelChoco(domains, constraints);
  }

  proc engine() {
    trap FoundSolution {
      par
      || input_order_lb();
      || propagation();
      || one_solution();
      end
    }
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

  proc one_solution() {
    loop {
      when consistent |= Consistent.True {
        exit FoundSolution;
      }
      pause;
    }
  }

  private static void modelChoco(VarStore domains,
    ConstraintStore constraints)
  {
    Model model = domains.model();
    IntVar[] vars = new IntVar[m*m];
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < m; j++) {
        vars[i * m + j] = (IntVar) domains.alloc(new IntDomain(0, m - 1, false));
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
        constraints.join(model.allDifferent(col, "AC"));
        constraints.join(model.allDifferent(row, "AC"));
    }
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

  private static void printNumberSolution(int obj) {
    System.out.println("Number of solutions: " + sol + "[obj = " + obj + "]");
  }

  private static int sol = 0;
  private static void incSolution() {
    sol = sol + 1;
  }
}