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

import java.lang.System;
import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.lattices.*;
import bonsai.statistics.*;
import benchmark.Config;

public class BenchStats
{
  ref single_time ES consistent;
  public BenchStats(ES consistent) {
    this.consistent = consistent;
  }

  public proc record() =
    module Node nodes = new Node();
    module FailNode fails = new FailNode(consistent);
    module SolutionNode solutions = new SolutionNode(consistent);
    par
    || run nodes.count()
    || run fails.count()
    || run solutions.count()
    || flow updateStats(nodes.value, fails.value, solutions.value) end
    end
  end

  private static void updateStats(LMax nodes, LMax fails, LMax solutions) {
    Config.current.nodes = nodes.unwrap();
    Config.current.fails = fails.unwrap();
    Config.current.solutions = solutions.unwrap();
  }
}
