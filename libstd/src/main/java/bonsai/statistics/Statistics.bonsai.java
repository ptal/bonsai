// Copyright 2018 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.statistics;

import java.lang.System;
import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.lattices.*;

public class Statistics
{
  ref single_time ES consistent;
  public Statistics(ES consistent) {
    this.consistent = consistent;
  }

  public proc count() =
    module Depth depth = new Depth();
    module Node nodes = new Node();
    module FailNode fails = new FailNode(write consistent);
    module SolutionNode solutions = new SolutionNode(write consistent);
    par
    || run depth.count()
    || run nodes.count()
    || run fails.count()
    || run solutions.count()
    || flow printStatistics(nodes.value, depth.value, fails.value, solutions.value) end
    end
  end

  private static void printStatistics(LMax nodes, LMax depth, LMax fails, LMax solutions) {
    System.out.println("[statistics] nodes: " + nodes + ", depth: " + depth
      + ", fails: " + fails + ", solutions: " + solutions);
  }
}
