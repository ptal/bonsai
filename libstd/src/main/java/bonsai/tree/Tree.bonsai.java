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

package bonsai.tree;

import bonsai.strategies.BoundedDepth;
import bonsai.statistics.Depth;

public class Tree
{
  ref single_space LMax max_depth;
  public Tree(LMax max_depth) {
    this.max_depth = max_depth;
  }

  // This class generates a binary tree of depth `max_depth`.
  public proc generate() =
    module BoundedDepth bd = new BoundedDepth(max_depth);
    par
    <> run raw_binary()
    <> run bd.bound()
    end
  end

  public flow raw_binary() =
    space nothing end;
    space nothing end
  end

  public proc vizualize() =
    module Depth depth = new Depth();
    par
    || flow printStar(depth.value) end
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
