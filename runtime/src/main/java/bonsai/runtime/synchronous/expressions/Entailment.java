// Copyright 2018 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.runtime.synchronous.expressions;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.lattices.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.variables.*;

// e |= e'
public class Entailment extends ASTNode implements Expression
{
  // We have the variables appearing in the left and right side of the entailment.
  // Note that constant are not represented in this class and directly compiled into the closure.
  private List<FreeAccess> leftVars;
  private List<FreeAccess> rightVars;
  private Function<ArrayList<Object>, ES> eval;
  private ExprResult result;

  public Entailment(List<FreeAccess> leftVars, List<FreeAccess> rightVars,
    Function<ArrayList<Object>, ES> eval) {
    this.leftVars = leftVars;
    this.rightVars = rightVars;
    this.eval = eval;
    this.result = new ExprResult();
  }

  private void prepareArgs(Layer layer, List<FreeAccess> accesses) {
    for (FreeAccess access: accesses) {
      access.prepareInstant(layer);
    }
  }

  public void prepareInstant(Layer layer) {
    this.result = new ExprResult();
    prepareArgs(layer, leftVars);
    prepareArgs(layer, rightVars);
  }

  private boolean evalArgs(Layer layer, List<FreeAccess> accesses,
   ArrayList<Object> args)
  {
    boolean isReadOnly = true;
    for (FreeAccess access : accesses) {
      Variable var = access.executeFree(layer);
      isReadOnly = isReadOnly && var.isReadable();
      args.add(var.value());
    }
    return isReadOnly;
  }

  // We check if the result of the entailment cannot change anymore in the current instant.
  // In this case, we promote unknown to false.
  private Kleene promoteResult(Kleene r, boolean leftReadOnly, boolean rightReadOnly) {
    Kleene promoted = null;
    if (r == Kleene.UNKNOWN && leftReadOnly && rightReadOnly) {
      promoted = Kleene.FALSE;
    }
    else if (r == Kleene.TRUE && rightReadOnly) {
      promoted = Kleene.TRUE;
    }
    else if (r == Kleene.FALSE && leftReadOnly) {
      promoted = Kleene.FALSE;
    }
    return promoted;
  }

  public ExprResult execute(Layer layer) {
    if (result.isSuspended()) {
      ArrayList<Object> args = new ArrayList();
      boolean leftReadOnly = evalArgs(layer, leftVars, args);
      boolean rightReadOnly = evalArgs(layer, rightVars, args);
      Kleene r = eval.apply(args).unwrap();
      Kleene promoted = promoteResult(r, leftReadOnly, rightReadOnly);
      if (promoted != null) {
        result = new ExprResult(new ES(promoted));
      }
    }
    return result;
  }

  public CanResult canWriteOn(String uid, boolean inSurface) {
    return new CanResult(true,false);
  }

  public void meetRWCounter(Layer layer) {}
}
