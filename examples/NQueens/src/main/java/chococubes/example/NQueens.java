package chococubes.example;

import java.lang.reflect.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import org.chocosolver.solver.variables.*;
import bonsai.chococubes.*;
import bonsai.chococubes.core.*;
import bonsai.chococubes.choco.*;
import bonsai.chococubes.sugarcubes.*;

public class NQueens
{
  public static void main(final String[] args) {
    Program p =
      new SpacetimeVar("domains", Spacetime.WorldLine, (env) -> VarStore.bottom(),
      new SpacetimeVar("constraints", Spacetime.WorldLine, (env) -> ConstraintStore.bottom(),
      declareNQueensVars(1, 4,
      printVariables())));
    SpaceMachine machine = SpaceMachine.create(p);
    try {
      machine.execute();
    } catch (Exception e) {
      e.printStackTrace();
    }
  }

  private static Program declareNQueensVars(int x, int n, Program body) {
    if (x > n) {
      return body;
    }
    else {
      return new LocationVar("queen"+n, "domains",
        (env) -> new IntDomain(1, n),
        declareNQueensVars(x + 1, n, body));
    }
  }

  private static Program printVariables() {
    return new ClosureAtom((env) -> {
        VarStore domains = (VarStore) env.var("domains");
        System.out.print("vars = [");
        for (IntVar v : domains.vars()) {
          System.out.print(v + ", ");
        }
        System.out.println("]");
      });
  }
}
