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

// Depth-bounded discrepancy search (DDS) improves ILDS by not requiring an upper bound on the depth of the search tree.
// It is available here:
//   T. Walsh, “Depth-bounded discrepancy search,” in IJCAI, 1997, vol. 97, pp. 1388–1393.
public class DDS
{
  // This variable represents the depth until which we can take discrepancies.
  ref single_space LMax max_dis_at_depth;

  public DDS(LMax max_dis_at_depth) {
    this.max_dis_at_depth = max_dis_at_depth;
  }

  public proc bound() =
    module Depth depth = new Depth();
    single_space LMin zero = new LMin(0);
    single_space LMin one = new LMin(1);
    par
    <> run depth.count();
    <> flow
         single_time LMin remaining_depth = minus(max_dis_at_depth, depth.value);
         par
           when remaining_depth |= zero then
             space nothing end;
             prune
           else
             when remaining_depth |= one then
               prune;
               space nothing end;
             else
               space nothing end;
             end
           end
         end
       end
    end
  end

  public static LMin minus(LMax a, LMax b) {
    return new LMin(a.unwrap() - b.unwrap());
  }
}
