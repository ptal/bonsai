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

package benchmark;

import benchmark.bonsai.*;
import benchmark.choco.*;

import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;

import java.util.*;
import java.util.function.*;
import org.chocosolver.solver.Model;

public class Benchmark
{
  private boolean human = true;
  private int timeLimitSeconds = 1080;

  private List<Integer> paramsNQueens = Arrays.asList(7,8,9,10,11);//,11,12,13,14);
  private List<Integer> paramsGolombRuler = Arrays.asList(7,8,9,10,11);//,10,11,12,13,14);
  private List<Integer> paramsLatinSquare = Arrays.asList(30,35,40,45,50,55);

  public static void main(String[] args) {
    new Benchmark().start();
  }

  private void start() {
    Config.timeout = timeLimitSeconds;
    System.out.println(Config.headerCSV());
    // WARM-UP
    // runChocoNQueens(14);
    // Benches
    // benchNQueens();
    // benchLatinSquare();
    benchGolombRuler();
  }

  private void benchNQueens() {
    for (Integer n : paramsNQueens) {
      runBonsaiNQueens(n);
    }
    for (Integer n : paramsNQueens) {
      runBonsaiInlinedNQueens(n);
    }
    for (Integer n : paramsNQueens) {
      runChocoNQueens(n);
    }
  }

  private void benchGolombRuler() {
    // for (Integer n : paramsGolombRuler) {
    //   runBonsaiInlinedGolombRulerIOLB(n);
    // }
    for (Integer n : paramsGolombRuler) {
      runBonsaiGolombRulerIOLB(n);
    }
    // for (Integer n : paramsGolombRuler) {
    //   runBonsaiGolombRulerFFM(n);
    // }
    // for (Integer n : paramsGolombRuler) {
    //   runBonsaiGolombRulerMDLB(n);
    // }
    // for (Integer n : paramsGolombRuler) {
    //   runChocoGolombRulerIOLB(n);
    // }
    // for (Integer n : paramsGolombRuler) {
    //   runChocoGolombRulerFFM(n);
    // }
  }

  private void benchLatinSquare() {
    for (Integer n : paramsLatinSquare) {
      runBonsaiLatinSquare(n);
    }
    for (Integer n : paramsLatinSquare) {
      runChocoLatinSquare(n);
    }
  }

  private void runBonsaiNQueens(int n) {
    runBonsai("NQueens", n, (p) -> p.nqueensWithStats(), (p) -> p.nqueens());
  }

  private void runBonsaiInlinedNQueens(int n) {
    runBonsai("InlinedNQueens", n, (p) -> p.inlined_nqueens(), (p) -> p.inlined_nqueens());
  }

  private void runBonsaiGolombRulerIOLB(int n) {
    runBonsai("GolombRulerIOLB", n, (p) -> p.golombRulerIOLBWithStats(), (p) -> p.golombRulerIOLB());
  }

  private void runBonsaiInlinedGolombRulerIOLB(int n) {
    runBonsai("InlinedGolombRulerIOLB", n, (p) -> p.inlined_golomb_iolb(), (p) -> p.inlined_golomb_iolb());
  }

  private void runBonsaiGolombRulerFFM(int n) {
    runBonsai("GolombRulerFFM", n, (p) -> p.golombRulerFFMWithStats(), (p) -> p.golombRulerFFM());
  }

  private void runBonsaiGolombRulerMDLB(int n) {
    runBonsai("GolombRulerMDLB", n, (p) -> p.golombRulerMDLBWithStats(), (p) -> p.golombRulerMDLB());
  }

  private void runBonsaiLatinSquare(int n) {
    runBonsai("LatinSquare", n, (p) -> p.latinSquareWithStats(), (p) -> p.latinSquare());
  }

  private void runBonsai(String name, int n,
   Function<Problems, Statement> processWithStats, Function<Problems, Statement> process)
  {
    Config.init(name, n);
    // We test if the current instance is within the time limit, and we compute some statistics such as the number of nodes.
    SpaceMachine<Problems> machine;
    try {
      machine = new SpaceMachine<>(new Problems(), processWithStats, false);
      machine.execute();
    }
    catch (TimeLimitException e) {
      System.out.println(Config.current.toCSV(human));
      return;
    }
    System.out.println(Config.current.toCSV(human));
  }

  private void runChoco(String name, int n, Supplier<Boolean> model) {
    Config.init(name, n);
    try {
      model.get();
    }
    catch (TimeLimitException e) {
      System.out.println(Config.current.toCSV(human));
      return;
    }
    System.out.println(Config.current.toCSV(human));
  }

  private void runChocoNQueens(int n) {
    runChoco("NQueensChoco", n, () -> {new NQueensModel().solve(); return true; });
  }

  private void runChocoGolombRulerIOLB(int n) {
    runChoco("GolombRulerChocoIOLB", n, () -> {new GolombRulerModel().solve(); return true; });
  }

  private void runChocoGolombRulerFFM(int n) {
    runChoco("GolombRulerChocoFFM", n, () -> {new GolombRulerModel(true).solve(); return true; });
  }

  private void runChocoLatinSquare(int n) {
    runChoco("LatinSquareChoco", n, () -> {new LatinSquareModel().solve(); return true; });
  }
}
