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

package bonsai.cp;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;
import bonsai.runtime.core.*;
import bonsai.runtime.lattices.*;

import bonsai.runtime.lattices.choco.*;
import org.chocosolver.solver.variables.*;

public class MinimizeBAB
{
  ref world_line ConstraintStore constraints;
  ref single_time ES consistent;
  ref single_space IntVar x;

  public single_space LMin obj = bot;
  single_space LMax objV = new LMax(0);
  world_line LMax conV = new LMax(1);

  public MinimizeBAB(ConstraintStore constraints, ES consistent, IntVar x) {
    this.constraints = constraints;
    this.consistent = consistent;
    this.x = x;
  }

  public proc solve() =
    par run minimize() <> run yield_objective() end

  proc minimize() =
    loop
      when consistent |= true then
        when true |= consistent then
          single_space LMin pre_obj = new LMin(x.getLB());
          pause;
          obj <- pre_obj;
          readwrite objV.inc();
        else pause end
      else pause end
    end

  flow yield_objective() =
    when objV |= conV then
      when conV |= objV then nothing
      else
        constraints <- x.lt(obj.unwrap());
        single_time LMax objV2 = new LMax(objV.unwrap());
        space conV <- objV2; end;
      end
    end
  end
}
