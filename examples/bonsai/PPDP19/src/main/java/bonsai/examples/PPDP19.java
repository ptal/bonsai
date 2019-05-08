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

public class PPDP19
{
  public static void main(String[] args) {
    section3BoundedDepth();
  }

  // The code is slightly different than in the paper (which is simplified for clarity).
  // We must initialize the field "limit" of the class "BoundedTree" outside of the constructor.
  // The interface with "SpaceMachine" takes the module creating the program, and the process constructor separately.
  static void section3BoundedDepth() {
    System.out.println("\n  Section 3. Demo of iterative deepening search as appearing in Section 3.");
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
}
