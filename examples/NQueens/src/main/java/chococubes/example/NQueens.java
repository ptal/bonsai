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
    SpaceMachine machine = SpaceMachine.create(p);
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
          oneSolution()
        )));
  }

  // fn first_fail_middle() {
  //   let var in single_space = new FirstFail(domains.model());
  //   let val in single_space = new IntDomainMin();
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
      new SpacetimeVar("val", Spacetime.SingleSpace, (env) -> new IntDomainMin(),
      SC.loop(
        SC.seq(
        SC.when(new EntailmentConfig("consistent", (env) -> Consistent.Unknown),
          new SpacetimeVar("x", Spacetime.SingleTime, (env) -> {
            FirstFail var = (FirstFail) env.var("var");
            IntDomainMin val = (IntDomainMin) env.var("val");
            VarStore domains = (VarStore) env.var("domains");
            return var.getVariable(domains.vars());
          },
          new SpacetimeVar("mid", Spacetime.SingleTime, (env) -> {
            IntVar x = (IntVar) env.var("x");
            IntDomainMin val = (IntDomainMin) env.var("val");
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

  // fn propagation() {
  //   loop {
  //     consistent <- PropagatorEngine.propagate(domains, constraints);
  //     pause;
  //   }
  // }
  private static Program propagation() {
    return SC.loop(
      SC.seq(
        new Tell("consistent", (env) -> {
          VarStore domains = (VarStore) env.var("domains");
          ConstraintStore constraints = (ConstraintStore) env.var("constraints");
          Consistent consistent = PropagatorEngine.propagate(domains, constraints);
          return consistent;
        }),
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
  //   printVariables(domains);
  // }
  private static Program model() {
    return declareNQueensVars(1, 4,
    declareNQueensConstraint(
    printVariables()));
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

  private static Program declareNQueensConstraint(Program body) {
    return SC.seq(
      new Tell("constraints", (env) -> {
        VarStore domains = (VarStore) env.var("domains");
        return new AllDifferent(domains.vars(), "DEFAULT");
      }),
      body
    );
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
