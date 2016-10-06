package chococubes.example;

import java.util.*;
import java.lang.reflect.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.constraints.nary.alldifferent.*;
import org.chocosolver.solver.search.strategy.selectors.variables.*;
import org.chocosolver.solver.search.strategy.selectors.values.*;
import bonsai.chococubes.core.*;
import bonsai.chococubes.choco.*;
import bonsai.chococubes.sugarcubes.*;

public class NQueens
{
  public static void main(final String[] args) {
    Program p =
      new SpacetimeVar("domains", Spacetime.WorldLine, (env) -> VarStore.bottom(),
      new SpacetimeVar("constraints", Spacetime.WorldLine, (env) -> ConstraintStore.bottom(),
      SC.seq(
        model(),
        engine())));
    SpaceMachine machine = SpaceMachine.createDebug(p);
    try {
      machine.execute();
    } catch (Exception e) {
      e.printStackTrace();
    }
  }

  // fn engine() {
  //   trap FoundSolution {
  //     par
  //     || fail_first_middle();
  //     || propagation();
  //     || one_solution();
  //     end
  //   }
  // }
  private static Program engine() {
    return
      new SpacetimeVar("consistent", Spacetime.SingleTime, (env) -> FlatLattice.bottom(),
      SC.until("FoundSolution",
        SC.merge(
          failFirstMiddle(),
          propagation(),
          // oneSolution(),
          printSolution()
        )));
  }

  // fn first_fail_middle() {
  //   let var in single_space = new FirstFail(domains.model());
  //   let val in single_space = new IntDomainMiddle(IntDomainMiddle.FLOOR);
  //   loop {
  //     when consistent |= Consistent.Unknown {
  //       let x: IntVar in single_time = var.getVariable(domains.vars());
  //       let mid: int in single_time = val.selectValue(domains[x]);
  //       space
  //       || constraints[domains] <- x.gt(mid);
  //       || constraints[domains] <- x.lte(mid);
  //       end
  //     }
  //     pause;
  //   }
  // }
  private static Program failFirstMiddle() {
    return
      new SpacetimeVar("var", Spacetime.SingleSpace, (env) -> {
        VarStore domains = (VarStore) env.var("domains");
        return new FirstFail(domains.model());
      },
      new SpacetimeVar("val", Spacetime.SingleSpace, (env) -> new IntDomainMiddle(IntDomainMiddle.FLOOR),
      SC.loop(
        SC.seq(
        SC.when(new EntailmentConfig("consistent", (env) -> Consistent.Unknown),
          new SpacetimeVar("x", Spacetime.SingleTime, (env) -> {
            FirstFail var = (FirstFail) env.var("var");
            IntDomainMiddle val = (IntDomainMiddle) env.var("val");
            VarStore domains = (VarStore) env.var("domains");
            return var.getVariable(domains.vars());
          },
          new SpacetimeVar("mid", Spacetime.SingleTime, (env) -> {
            IntVar x = (IntVar) env.var("x");
            IntDomainMiddle val = (IntDomainMiddle) env.var("val");
            VarStore domains = (VarStore) env.var("domains");
            return new Integer(val.selectValue((IntVar) domains.index(x)));
          },
          new Space(new ArrayList<>(Arrays.asList(
            new SpaceBranch(new Tell("constraints", (env) -> {
              IntVar x = (IntVar) env.var("x");
              Integer mid = (Integer) env.var("mid");
              return x.gt(mid);
            })),
            new SpaceBranch(new Tell("constraints", (env) -> {
              IntVar x = (IntVar) env.var("x");
              Integer mid = (Integer) env.var("mid");
              return x.le(mid);
            }))
          ))))),
          SC.nothing()),
        SC.stop())
      )));
  }

  // fn one_solution() {
  //   loop {
  //     when consistent |= Consistent.True {
  //       exit FoundSolution;
  //     }
  //     pause;
  //   }
  // }
  private static Program oneSolution() {
    return SC.loop(
      SC.seq(
        SC.when(new EntailmentConfig("consistent", (env) -> Consistent.True),
          SC.generate("FoundSolution"),
          SC.nothing()),
        SC.stop()
    ));
  }

  // fn print_solution() {
  //   loop {
  //     when consistent |= Consistent.True {
  //       print_model();
  //     }
  //     pause;
  //   }
  // }
  private static Program printSolution() {
    return SC.loop(
      SC.seq(
        SC.when(new EntailmentConfig("consistent", (env) -> Consistent.True),
          printModel("Solution"),
          SC.nothing()),
        SC.stop()
    ));
  }

  // fn propagation() {
  //   loop {
  //     consistent <- PropagatorEngine.propagate(domains, constraints);
  //     pause;
  //   }
  // }
  private static Program propagation() {
    return SC.loop(
      SC.seq(
        // printModel("Before propagation"),
        new Tell("consistent", (env) -> {
          VarStore domains = (VarStore) env.var("domains");
          ConstraintStore constraints = (ConstraintStore) env.var("constraints");
          Consistent consistent = PropagatorEngine.propagate(domains, constraints);
          return consistent;
        }),
        // printModel("After propagation"),
        SC.stop()
    ));
  }

  // fn model() {
  //   let queen1: IntVar = domains <- new IntDomain(1,4);
  //   let queen2: IntVar = domains <- new IntDomain(1,4);
  //   let queen3: IntVar = domains <- new IntDomain(1,4);
  //   let queen4: IntVar = domains <- new IntDomain(1,4);
  //
  //   constraints[domains] <- new AllDifferent(domains.vars(), "DEFAULT");
  // }
  private static Program model() {
    return declareNQueensVars(1, 4,
    declareNQueensConstraint(
    SC.nothing()));
  }

  private static Program declareNQueensVars(int x, int n, Program body) {
    if (x > n) {
      return body;
    }
    else {
      return new LocationVar("queen"+x, "domains",
        (env) -> new IntDomain(1, n),
        declareNQueensVars(x + 1, n, body));
    }
  }

  private static Program declareNQueensConstraint(Program body) {
    return SC.seq(
      new Tell("constraints", (env) -> {
        VarStore domains = (VarStore) env.var("domains");
        return new AllDifferent(domains.vars(), "DEFAULT");
      }),
      body
    );
  }

  private static Program printModel(String message) {
    return new ClosureAtom((env) -> {
      VarStore domains = (VarStore) env.var("domains");
      System.out.print("["+message+"]" + domains.model());
    });
  }
}
