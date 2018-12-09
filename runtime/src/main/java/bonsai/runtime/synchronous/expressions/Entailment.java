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
  // Note that constants are not represented in this class and directly compiled into the closure.
  private final List<FreeAccess> leftVars;
  private final List<FreeAccess> rightVars;
  private final Function<ArrayList<Object>, Kleene> eval;
  private ExprResult result;

  public Entailment(List<FreeAccess> leftVars, List<FreeAccess> rightVars,
    Function<ArrayList<Object>, Kleene> eval) {
    this.leftVars = leftVars;
    this.rightVars = rightVars;
    this.eval = eval;
    this.result = new ExprResult();
  }

  public Entailment copy() {
    return new Entailment(
      ASTNode.copyList(leftVars),
      ASTNode.copyList(rightVars),
      eval);
  }

  private boolean evalArgs(Layer layer, String readOnlyHypothesis, List<FreeAccess> accesses,
   ArrayList<Object> args)
  {
    boolean isReadOnly = true;
    for (FreeAccess access : accesses) {
      Variable var = access.executeFree(layer);
      isReadOnly = isReadOnly && (var.isReadable() || var.uid().equals(readOnlyHypothesis));
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

  private void canInstantArgs(Layer layer, List<FreeAccess> accesses) {
    for (FreeAccess access: accesses) {
      access.canInstant(layer);
    }
  }

  public void canInstant(Layer layer) {
    this.result = new ExprResult();
    canInstantArgs(layer, leftVars);
    canInstantArgs(layer, rightVars);
  }

  private void terminateArgs(Layer layer, List<FreeAccess> accesses) {
    for (FreeAccess access: accesses) {
      access.terminate(layer);
    }
  }

  public void terminate(Layer layer) {
    terminateArgs(layer, leftVars);
    terminateArgs(layer, rightVars);
  }

  public ExprResult execute(Layer layer) {
    if (result.isSuspended()) {
      Kleene promoted = execute(layer, "");
      if (promoted != null) {
        commit(promoted);
      }
    }
    return result;
  }

  public Kleene execute(Layer layer, String readOnlyHypothesis) {
    ArrayList<Object> args = new ArrayList();
    boolean leftReadOnly = evalArgs(layer, readOnlyHypothesis, leftVars, args);
    boolean rightReadOnly = evalArgs(layer, readOnlyHypothesis, rightVars, args);
    Kleene r = eval.apply(args);
    Kleene promoted = promoteResult(r, leftReadOnly, rightReadOnly);
    return promoted;
  }

  public void commit(Kleene partialRes) {
    result = new ExprResult(new ES(partialRes));
  }

  public boolean canWriteOn(String uid) {
    return false;
  }
}
