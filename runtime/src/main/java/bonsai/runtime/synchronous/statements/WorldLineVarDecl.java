// Copyright 2018 Pierre Talbot

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
import bonsai.runtime.synchronous.exceptions.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.variables.*;

public class WorldLineVarDecl extends SingleSpaceVarDecl
{
  public WorldLineVarDecl(String uid, Expression initValue, Statement body) {
    super(uid, initValue, body);
  }

  public WorldLineVarDecl copy() {
    // throw new CannotCopyException("WorldLineVarDecl");
    return new WorldLineVarDecl(uid, initValue.copy(), body.copy());
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    checkExpressionStateEOI("Var decl", state1());
    return super.endOfInstant(layersRemaining, layer);
  }

  public StmtResult execute(int layersRemaining, Layer layer) {
    // Save the current variable before we remove it in case the body terminates (we still need to add it in the StmtResult).
    Variable v = layer.lookUpVar(uid);
    StmtResult res = super.execute(layersRemaining, layer);
    if (layersRemaining == 0 && !res.k.isInternal()) {
      res.registerWL(layer.currentQueue(), v, state3());
    }
    return res;
  }
}
