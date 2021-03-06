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
import bonsai.statistics.Discrepancy;

// This strategy explores the search tree until it reaches a limit for the number of discrepancies taken, it is described in the following paper:
//   W. D. Harvey and M. L. Ginsberg, “Limited discrepancy search,” in IJCAI (1), 1995, pp. 607–615.
// Note that this class only perform one iteration (given by limit).
public class BoundedDiscrepancy
{
  ref single_space LMax limit;
  public BoundedDiscrepancy(LMax limit) {
    this.limit = limit;
  }

  public proc bound() =
    module Discrepancy dis = new Discrepancy();
    par
    <> run dis.count();
    <> flow
        space nothing end;
        when dis.value |= limit then
          prune
        end
       end
    end
  end
}
