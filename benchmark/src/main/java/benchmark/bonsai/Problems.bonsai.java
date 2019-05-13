// Copyright 2019 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package benchmark.bonsai;

import bonsai.cp.Solver;
import bonsai.statistics.Statistics;

import benchmark.*;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;
import bonsai.runtime.core.*;
import bonsai.runtime.lattices.*;
import bonsai.runtime.lattices.choco.*;

import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.constraints.nary.alldifferent.*;

import org.chocosolver.solver.search.strategy.selectors.variables.*;
import org.chocosolver.solver.search.strategy.selectors.values.*;
import org.chocosolver.solver.*;
import org.chocosolver.solver.exception.ContradictionException;

public class Problems
{
  public proc nqueens() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par
      || module NQueens nqueens = new NQueens();
         run nqueens.solve()
      || run abortWhenTimeout()
      end
    end
  end

  public proc nqueensWithStats() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      module NQueens nqueens = new NQueens();
      module BenchStats stats = new BenchStats(write nqueens.consistent);
      par
      <> run nqueens.solve()
      <> run stats.record()
      <> run abortWhenTimeout()
      end
    end
  end

  public proc latinSquare() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par
      || module LatinSquare latinSquare = new LatinSquare();
         run latinSquare.solve()
      || run abortWhenTimeout()
      end
    end
  end

  public proc latinSquareWithStats() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      module LatinSquare latinSquare = new LatinSquare();
      module BenchStats stats = new BenchStats(write latinSquare.consistent);
      par
      <> run latinSquare.solve()
      <> run stats.record()
      <> run abortWhenTimeout()
      end
    end
  end

  public proc golombRulerIOLB() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par
      || module GolombRuler golombRuler = new GolombRuler();
         run golombRuler.solveIOLB()
      || run abortWhenTimeout()
      end
    end
  end

  public proc golombRulerIOLBWithStats() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      module GolombRuler golombRuler = new GolombRuler();
      module BenchStats stats = new BenchStats(write golombRuler.consistent);
      par
      <> run golombRuler.solveIOLB()
      <> run stats.record()
      <> run abortWhenTimeout()
      end
    end
  end

  public proc golombRulerFFM() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par
      || module GolombRuler golombRuler = new GolombRuler();
         run golombRuler.solveFFM()
      || run abortWhenTimeout()
      end
    end
  end

  public proc golombRulerFFMWithStats() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      module GolombRuler golombRuler = new GolombRuler();
      module BenchStats stats = new BenchStats(write golombRuler.consistent);
      par
      <> run golombRuler.solveFFM()
      <> run stats.record()
      <> run abortWhenTimeout()
      end
    end
  end

  public proc golombRulerMDLB() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par
      || module GolombRuler golombRuler = new GolombRuler();
         run golombRuler.solveMDLB()
      || run abortWhenTimeout()
      end
    end
  end

  public proc golombRulerMDLBWithStats() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      module GolombRuler golombRuler = new GolombRuler();
      module BenchStats stats = new BenchStats(write golombRuler.consistent);
      par
      <> run golombRuler.solveMDLB()
      <> run stats.record()
      <> run abortWhenTimeout()
      end
    end
  end

  private proc abortWhenTimeout() =
    single_space long start = currentTime();
    flow checkTime(start) end
  end

  public static long currentTime() {
    return System.nanoTime();
  }
  public static void checkTime(long start) {
    Config.current.time = System.nanoTime() - start;
    if (Config.current.hasTimedOut()) {
      throw new TimeLimitException();
    }
  }

  // To test the overhead of abstraction inside modules, we inlined all the processes in a single process.
  public proc inlined_golomb_iolb() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      single_space LMax nodes = new LMax(0);
      single_space LMax fails = new LMax(0);
      single_space LMax solutions = new LMax(0);
      single_space long start = currentTime();
      world_line VarStore domains = new VarStore();
      world_line ConstraintStore constraints = new ConstraintStore();
      single_time ES consistent = unknown;
      modelGolombRuler(write domains, write constraints);
      single_space IntVar x = rulerLengthVar(write domains);
      single_space LMin obj = bot;
      single_space VariableSelector<IntVar> var = inputOrder(write domains);
      single_space IntValueSelector val = min();
      par
        flow consistent <- updateBound(write domains, write x, read obj) end
      <>
        flow
          readwrite nodes.inc();
          consistent <- constraints.propagate(readwrite domains)
        end
      <>
        loop
          when consistent |= true then
            when true |= consistent then
              single_space LMin pre_obj = new LMin(x.getLB());
              readwrite solutions.inc();
              pause;
              obj <- pre_obj;
            else pause end
          else pause end
        end
      <>
        flow
          when unknown |= consistent then
            single_time IntVar y = readwrite var.getVariable(domains.vars());
            single_time Integer v = readwrite val.selectValue(y);
            space readwrite domains.join_eq(y, v) end;
            space readwrite domains.join_neq(y, v) end
          end
        end
      <>
        flow
          when consistent |= false then
            readwrite fails.inc();
            updateStats(start, nodes, fails, solutions)
          end
        end
      end
    end
  end

  private static void updateStats(long start, LMax nodes, LMax fails, LMax solutions) {
    Config.current.nodes = nodes.unwrap();
    Config.current.fails = fails.unwrap();
    Config.current.solutions = solutions.unwrap();
    checkTime(start);
  }

  public static VariableSelector<IntVar> inputOrder(VarStore domains) {
    return new InputOrder(domains.model());
  }

  public static VariableSelector<IntVar> firstFail(VarStore domains) {
    return new FirstFail(domains.model());
  }

  public static IntValueSelector min() {
    return new IntDomainMin();
  }

  public static ES updateBound(VarStore _domains, IntVar x, LMin obj) {
    Config.current.obj = obj.unwrap();
    try {
      x.updateUpperBound(obj.unwrap() - 1, Cause.Null);
      return new ES(Kleene.UNKNOWN);
    }
    catch (ContradictionException c) {
      return new ES(Kleene.FALSE);
    }
  }

  private static void modelGolombRuler(VarStore domains, ConstraintStore constraints)
  {
    int m = Config.current.n;
    IntVar[] ticks = new IntVar[m];
    IntVar[] diffs = new IntVar[(m*m -m)/2];
    Model model = domains.model();

    int ub =  (m < 31) ? (1 << (m + 1)) - 1 : 9999;
    for(int i=0; i < ticks.length; i++) {
      ticks[i] = (IntVar) domains.alloc(new VarStore.IntDomain(0, ub, true));
    }
    for(int i=0; i < diffs.length; i++) {
      diffs[i] = (IntVar) domains.alloc(new VarStore.IntDomain(0, ub, true));
    }

    constraints.join_in_place(model.arithm(ticks[0], "=", 0));
    for (int i = 0; i < m - 1; i++) {
      constraints.join_in_place(model.arithm(ticks[i + 1], ">", ticks[i]));
    }

    IntVar[][] m_diffs = new IntVar[m][m];
    for (int k = 0, i = 0; i < m - 1; i++) {
      for (int j = i + 1; j < m; j++, k++) {
        // d[k] is m[j]-m[i] and must be at least sum of first j-i integers
        // <cpru 04/03/12> it is worth adding a constraint instead of a view
        constraints.join_in_place(model.scalar(new IntVar[]{ticks[j], ticks[i]}, new int[]{1, -1}, "=", diffs[k]));
        constraints.join_in_place(model.arithm(diffs[k], ">=", (j - i) * (j - i + 1) / 2));
        constraints.join_in_place(model.arithm(diffs[k], "-", ticks[m - 1], "<=", -((m - 1 - j + i) * (m - j + i)) / 2));
        constraints.join_in_place(model.arithm(diffs[k], "<=", ticks[m - 1], "-", ((m - 1 - j + i) * (m - j + i)) / 2));
        m_diffs[i][j] = diffs[k];
      }
    }
    constraints.join_in_place(model.allDifferent(diffs, "BC"));

    // break symmetries
    if (m > 2) {
      constraints.join_in_place(model.arithm(diffs[0], "<", diffs[diffs.length - 1]));
    }
  }

  private static IntVar rulerLengthVar(VarStore domains) {
    int m = Config.current.n;
    return (IntVar)domains.model().getVars()[m - 1];
  }

  // To test the overhead of abstraction inside modules, we inlined all the processes in a single process.
  public proc inlined_nqueens() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      single_space LMax nodes = new LMax(0);
      single_space LMax fails = new LMax(0);
      single_space LMax solutions = new LMax(0);
      single_space long start = currentTime();
      world_line VarStore domains = new VarStore();
      world_line ConstraintStore constraints = new ConstraintStore();
      single_time ES consistent = unknown;
      modelNQueens(write domains, write constraints);
      single_space VariableSelector<IntVar> var = firstFail(write domains);
      single_space IntValueSelector val = min();
      par
        flow
          readwrite nodes.inc();
          consistent <- constraints.propagate(readwrite domains)
        end
      <>
        flow
          when unknown |= consistent then
            single_time IntVar y = readwrite var.getVariable(domains.vars());
            single_time Integer v = readwrite val.selectValue(y);
            space readwrite domains.join_eq(y, v) end;
            space readwrite domains.join_neq(y, v) end
          else
            when consistent |= false then
              readwrite fails.inc();
            else
              readwrite solutions.inc();
            end;
            updateStats(start, nodes, fails, solutions);
          end
        end
      end
    end
  end

  private static void modelNQueens(VarStore domains,
    ConstraintStore constraints)
  {
    int n = Config.current.n;
    IntVar[] vars = new IntVar[n];
    IntVar[] diag1 = new IntVar[n];
    IntVar[] diag2 = new IntVar[n];
    for(int i = 0; i < n; i++) {
      vars[i] = (IntVar) domains.alloc(new VarStore.IntDomain(1, n, false));
      diag1[i] = domains.model().intOffsetView(vars[i], i);
      diag2[i] = domains.model().intOffsetView(vars[i], -i);
    }
    constraints.join_in_place(new AllDifferent(vars, "BC"));
    constraints.join_in_place(new AllDifferent(diag1, "BC"));
    constraints.join_in_place(new AllDifferent(diag2, "BC"));
  }
}
