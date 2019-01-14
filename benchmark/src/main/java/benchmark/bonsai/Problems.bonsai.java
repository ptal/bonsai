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
      || single_space long start = currentTime();
         flow checkTime(start) end
      end
    end
  end

  public proc nqueensWithStats() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par
      || module NQueens nqueens = new NQueens();
         run nqueens.solveWithStats()
      || single_space long start = currentTime();
         flow checkTime(start) end
      end
    end
  end

  public proc latinSquare() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par
      || module LatinSquare latinSquare = new LatinSquare();
         run latinSquare.solve()
      || single_space long start = currentTime();
         flow checkTime(start) end
      end
    end
  end

  public proc latinSquareWithStats() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par
      || module LatinSquare latinSquare = new LatinSquare();
         run latinSquare.solveWithStats()
      || single_space long start = currentTime();
         flow checkTime(start) end
      end
    end
  end

  public proc golombRuler() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par
      || module GolombRuler golombRuler = new GolombRuler();
         run golombRuler.solve()
      || single_space long start = currentTime();
         flow checkTime(start) end
      end
    end
  end

  public proc golombRulerWithStats() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par
      || module GolombRuler golombRuler = new GolombRuler();
         run golombRuler.solveWithStats()
      || single_space long start = currentTime();
         flow checkTime(start) end
      end
    end
  end

  public proc golombRulerReverse() =
    single_space StackRL stack = new StackRL();
    universe with stack in
      par
      || module GolombRuler golombRuler = new GolombRuler();
         run golombRuler.solve()
      || single_space long start = currentTime();
         flow checkTime(start) end
      end
    end
  end

  public proc golombRulerWithStatsReverse() =
    single_space StackRL stack = new StackRL();
    universe with stack in
      par
      || module GolombRuler golombRuler = new GolombRuler();
         run golombRuler.solveWithStats()
      || single_space long start = currentTime();
         flow checkTime(start) end
      end
    end
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
