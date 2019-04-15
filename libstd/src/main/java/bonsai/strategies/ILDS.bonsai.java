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

package bonsai.strategies;

import bonsai.runtime.lattices.*;
import bonsai.statistics.Depth;
import bonsai.statistics.Discrepancy;

import java.lang.System;

// This strategy improves "BoundedDiscrepancy" by not reexploring leaves already explored at a former iteration.
//   R. E. Korf, “Improved Limited Discrepancy Search,” in In Proceedings of AAAI-96, 1996, pp. 286–291.
// Note that this class only perform one iteration (given by limit).
// ILDS has a drawback however: it needs the depth of the search tree.
// To obtain ILDS, you must combine this strategy with DiscrepancySearch.
public class ILDS
{
  ref single_space LMax max_discrepancy;
  ref single_space LMax max_depth;

  public ILDS(LMax max_discrepancy, LMax max_depth) {
    this.max_discrepancy = max_discrepancy;
    this.max_depth = max_depth;
  }

  public proc bound() =
    module Depth depth = new Depth();
    module Discrepancy dis = new Discrepancy();
    par
    <> run depth.count();
    <> run dis.count();
    <> flow
         single_time LMin remaining_depth = minus(max_depth, depth.value);
         single_time LMin remaining_dis = minus(max_discrepancy, dis.value);
         when remaining_depth |= remaining_dis then
           prune
         end;
         space nothing end;
       end
    end
  end

  public static LMin minus(LMax a, LMax b) {
    return new LMin(a.unwrap() - b.unwrap());
  }
}
