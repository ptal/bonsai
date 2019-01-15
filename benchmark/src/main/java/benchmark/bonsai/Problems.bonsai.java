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
}
