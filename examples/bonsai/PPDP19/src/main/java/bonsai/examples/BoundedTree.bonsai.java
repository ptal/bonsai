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

import bonsai.runtime.queueing.*;
import bonsai.runtime.lattices.*;
import java.lang.System;

public class BoundedTree
{
  public single_space LMax limit = new LMax(0);

  public proc bounded_tree =
    module Tree tree = new Tree();
    module BoundedDepth bd = new BoundedDepth(limit);
    par
    <> run tree.binary()
    <> run bd.bound_depth()
    <> run vizualize()
    end
  end

  public proc vizualize =
    module Depth viz_depth = new Depth();
    par
    <> run viz_depth.count()
    <> flow printStar(viz_depth.depth) end
    end
  end

  public proc binary_stats =
    module Tree generator = new Tree();
    module Depth depth = new Depth();
    par
    || run generator.binary()
    || run depth.count()
    end
  end

  public static void printStar(LMax spaces) {
    for(int i=0; i < spaces.unwrap(); i++) {
      System.out.print("  ");
    }
    System.out.println("*");
  }
}
