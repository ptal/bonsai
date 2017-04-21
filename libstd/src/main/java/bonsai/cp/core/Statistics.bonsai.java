// Copyright 2017 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.cp.core;

import java.util.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;
import bonsai.runtime.core.*;
import bonsai.runtime.choco.*;
import bonsai.runtime.sugarcubes.*;

public class Statistics implements Resettable<Statistics>
{
  public single_space transient L<BInteger> nodes = bot;
  public world_line transient W<BInteger> depth = bot;

  public void reset(Statistics s) {
    this.nodes.reset(s.nodes);
    this.depth.reset(s.depth);
  }

  public proc execute() {
    par
    || countNode();
    || countDepth();
    end
  }

  public proc countDepth() {
    depth <- new BInteger(0);
    loop {
      pause;
      depth <- inc(pre depth);
    }
  }

  public proc countNode() {
    nodes <- new BInteger(1);
    loop {
      pause;
      nodes <- inc(pre nodes);
    }
  }

  public proc print() {
    ~printStats(nodes, depth);
  }

  private BInteger inc(L<BInteger> x) {
    int n = x.unwrap().value + 1;
    return new BInteger(n);
  }

  private void printStats(L<BInteger> nodes, W<BInteger> depth) {
    System.out.println("Nodes: " + nodes);
    System.out.println("Current depth: " + depth);
  }
}
