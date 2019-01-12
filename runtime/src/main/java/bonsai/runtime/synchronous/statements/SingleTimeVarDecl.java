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
import bonsai.runtime.synchronous.exceptions.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class SingleTimeVarDecl extends VarDecl implements Statement
{
  private Consumer<Object> refUpdater;

  // For local single_time variable.
  public SingleTimeVarDecl(String uid, Expression initValue, Statement body) {
    this(uid, initValue, (Object o) -> {}, body);
  }

  // For field single_time variable.
  public SingleTimeVarDecl(String uid, Expression initValue,
   Consumer<Object> refUpdater, Statement body)
  {
    super(uid, initValue, body);
    this.refUpdater = refUpdater;
  }

  public SingleTimeVarDecl copy() {
    // throw new CannotCopyException("SingleTimeVarDecl");
    return new SingleTimeVarDecl(uid.get(), initValue.copy(), body.copy());
  }

  protected void enterScope(Layer layer) {
    layer.enterScope(uid.get(), exprResult.unwrap(), refUpdater);
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    CompletionCode k = super.endOfInstant(layersRemaining, layer);
    if (layersRemaining == 0 && state2()) {
      terminate(layer);
    }
    return k;
  }

  public void canInstant(int layersRemaining, Layer layer) {
    if(layersRemaining == 0) {
      // System.out.println("SingleTimeVarDecl.canInstant: register " + uid);
      layer.register(uid.get(), true);
      initValue.canInstant(layer);
      exprResult = new ExprResult();
    }
    body.canInstant(layersRemaining, layer);
  }
}
