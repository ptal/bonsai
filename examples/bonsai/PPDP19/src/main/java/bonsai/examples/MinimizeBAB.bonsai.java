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

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;
import bonsai.runtime.core.*;
import bonsai.runtime.lattices.*;

import bonsai.runtime.lattices.choco.*;
import org.chocosolver.solver.variables.*;
import org.chocosolver.solver.Cause;
import org.chocosolver.solver.exception.ContradictionException;

public class MinimizeBAB
{
  ref world_line VarStore domains;
  ref single_time ES consistent;
  ref single_space IntVar x;

  public single_space LMin obj = bot;

  public MinimizeBAB(VarStore domains, ES consistent, IntVar x) {
    this.domains = domains;
    this.consistent = consistent;
    this.x = x;
  }

  public proc solve =
    par run minimize() <> run yield_objective() end

  proc minimize =
    loop
      when consistent |= true then
        when true |= consistent then
          single_space LMin pre_obj = new LMin(x.getLB());
          pause;
          obj <- pre_obj;
        else pause end
      else pause end
    end

  flow yield_objective =
    consistent <- updateBound(write domains, write x, read obj)
  end

  private static ES updateBound(VarStore _domains, IntVar x, LMin obj) {
    try {
      if (!obj.isBottom()) {
        x.updateUpperBound(obj.unwrap() - 1, Cause.Null);
      }
      return new ES(Kleene.UNKNOWN);
    }
    catch (ContradictionException c) {
      return new ES(Kleene.FALSE);
    }
  }
}
