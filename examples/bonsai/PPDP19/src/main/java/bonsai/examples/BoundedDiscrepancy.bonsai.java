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

import bonsai.runtime.lattices.*;
import java.lang.System;

public class BoundedDiscrepancy {
  world_line LMax dis = new LMax(0);
  ref single_space LMax limit;
  public BoundedDepth(LMax limit) {
    this.limit = limit;
  }
  public flow bound_dis =
    space nothing end;
    when dis |= limit then
      prune
    else
      space readwrite dis.inc() end
    end
  end
}