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

package bonsai.runtime.synchronous.statements;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.expressions.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class SingleSpaceVarDecl extends ASTNode implements Program
{
  private String uid;
  private Expression initValue;
  private Program body;

  private CompletionCode result;
  private ExprResult exprResult;

  public SingleSpaceVarDecl(String uid, Expression initValue, Program body) {
    this.uid = uid;
    this.initValue = initValue;
    this.body = body;
    this.result = CompletionCode.WAIT;
    this.exprResult = new ExprResult();
  }

  // in the initializing expression.
  private boolean state1() {
    return result != CompletionCode.TERMINATE && exprResult.isSuspended();
  }

  // in the body.
  private boolean state2() {
    return result != CompletionCode.TERMINATE && !exprResult.isSuspended();
  }

  // terminated.
  private boolean state3() {
    return result == CompletionCode.TERMINATE;
  }

  public void prepareSub(Environment env, int layerIndex) {
    body.prepareSub(env, layerIndex);
  }
  public CompletionCode executeSub(Environment env, int layerIndex) {
    return body.executeSub(env, layerIndex);
  }

  public void prepare(Layer layer) {
    result = CompletionCode.WAIT;
    exprResult = new ExprResult();
    initValue.prepare(layer);
    body.prepare(layer);
  }

  public CompletionCode execute(Layer layer) {
    executeState1(layer);
    executeState2(layer);
    return result;
  }

  private void executeState1(Layer layer) {
    if (state1()) {
      exprResult = initValue.execute(layer);
      if (!exprResult.isSuspended()) {
        initValue.terminate(layer);
        layer.enterScope(uid, exprResult.unwrap(), (Object o) -> {});
      }
    }
  }

  private void executeState2(Layer layer) {
    if (state2()) {
      result = body.execute(layer);
      executeState3(layer);
    }
  }

  // This state must be only executed one time once `state3()` holds (it is not idempotent).
  private void executeState3(Layer layer) {
    if (state3()) {
      layer.exitScope(uid);
      // Free the pointed value of the variable.
      exprResult = new ExprResult();
    }
  }

  private void jumpState3(Layer layer) {
    result = CompletionCode.TERMINATE;
    executeState3(layer);
  }

  public CanResult canWriteOn(String uid, boolean inSurface) {
    CanResult result = CanResult.IDENTITY;
    if (state1()) {
      result = initValue.canWriteOn(uid, inSurface);
    }
    if (state1() || state2()) {
      result = result.and_term(body.canWriteOn(uid, inSurface));
    }
    return result;
  }

  public boolean canAnalysis(Layer layer) {
    return initValue.canAnalysis(layer) && body.canAnalysis(layer);
  }

  public boolean terminate(Layer layer) {
    boolean canTerminate = true;
    if (!state3()) {
      if (state1()) {
        initValue.terminate(layer);
      }
      if (state1() || state2()) {
        canTerminate = canTerminate && body.terminate(layer);
      }
      jumpState3(layer);
    }
    return canTerminate;
  }
}
