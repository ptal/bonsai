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

package bonsai.examples;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.lattices.*;
import bonsai.runtime.lattices.choco.*;

import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.constraints.nary.alldifferent.*;

public class PPDP19
{
  public static void main(String[] args) {
    PPDP19 demo = new PPDP19();
    demo.beginMessage();
    demo.section3BoundedDepth();
    demo.section5CSP();
    demo.section5LDS();
    demo.section5LDS_IDS();
    demo.endMessage();
  }

  // The code is slightly different than in the paper (which is simplified for clarity).
  // We must initialize the field "limit" of the class "BoundedTree" outside of the constructor.
  // The interface with "SpaceMachine" takes the module creating the program, and the process constructor separately.
  void section3BoundedDepth() {
    System.out.println("\n  Section 3. Demo of iterative deepening search (IDS).");
    System.out.println("  =========\n");

    for(int limit = 0; limit < 3; limit++) {
      System.out.println("Limit = " + limit);
      BoundedTree program = new BoundedTree();
      program.limit = new LMax(limit);
      StackLR queue = new StackLR();
      SpaceMachine<BoundedTree> machine = new SpaceMachine<>(program, (p) -> p.bounded_tree(), queue);
      machine.execute();
    }
  }

  // We illustrate the section 5.1 with the NQueen CSP.
  void section5CSP() {
    System.out.println("\n  Section 5.1. Demo of CSP state space exploration (3 first solutions of the 8-Queens problem).");
    System.out.println("  ===========\n");
    Solver solver = new Solver();
    StackLR queue = new StackLR();
    SpaceMachine<Solver> machine = new SpaceMachine<>(solver, (p) -> p.nqueens(), queue);
    for(int i = 0; i < 3; i++) {
      machine.execute();
    }
  }

  void section5LDS() {
    System.out.println("\n  Section 5.2. Demo of limited-discrepancy search (LDS).");
    System.out.println("  ===========\n");

    for(int limit = 0; limit < 3; limit++) {
      System.out.println("Limit = " + limit);
      BoundedTree program = new BoundedTree();
      program.limit = new LMax(limit);
      StackLR queue = new StackLR();
      SpaceMachine<BoundedTree> machine = new SpaceMachine<>(program, (p) -> p.bounded_dis(), queue);
      machine.execute();
    }
  }

  void section5LDS_IDS() {
    System.out.println("\n  Section 5.3. Demo of LDS+IDS.");
    System.out.println("  ===========\n");

    for(int limit = 0; limit < 3; limit++) {
      System.out.println("Limit = " + limit);
      BoundedTree program = new BoundedTree();
      program.limit = new LMax(limit);
      StackLR queue = new StackLR();
      SpaceMachine<BoundedTree> machine = new SpaceMachine<>(program, (p) -> p.bounded_depth_dis(), queue);
      machine.execute();
    }
  }

  void beginMessage() {
    System.out.println("\n >>>> Welcome to the demo of the code presented in the paper submitted to PPDP19. <<<<\n");
    System.out.println("Note: The tree are drawn as follows: a '*' represents one node, and the spaces before '*' represent the depth of this node.");
    System.out.println("For additional strategies including DDS and ILDS, please see libstd/ in the package `bonsai.strategies.*`.\n");
  }

  void endMessage() {
    System.out.println("\n    Thanks for watching!");
  }
}
