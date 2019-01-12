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

package bonsai.runtime.synchronous.statements;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.expressions.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.exceptions.*;
import bonsai.runtime.synchronous.env.*;

/// A module declaration does not have a UID because it is not registered in the space.
/// However, we register its reference fields.
/// The other fields are declared in `body`.
public class ModuleVarDecl extends VarDecl implements Statement
{
  public static class ReferenceField {
    public String uid;
    public Consumer<Object> refUpdater;
    public Supplier<Object> fieldAccess;
    public ReferenceField(String uid, Consumer<Object> refUpdater,
     Supplier<Object> fieldAccess)
    {
      this.uid = uid;
      this.refUpdater = refUpdater;
      this.fieldAccess = fieldAccess;
    }
  }

  private final List<ReferenceField> refFields;

  public ModuleVarDecl(List<ReferenceField> refFields,
   Expression initValue, Statement body) {
    super(initValue, body);
    this.refFields = refFields;
  }

  protected void enterScope(Layer layer) {}

  public ModuleVarDecl copy() {
    return new ModuleVarDecl(refFields,
      initValue.copy(), body.copy());
  }

  public void canInstant(int layersRemaining, Layer layer) {
    if(layersRemaining == 0) {
      if (state1()) {
        initValue.canInstant(layer);
      }
    }
    body.canInstant(layersRemaining, layer);
  }

  protected boolean executeState1(Layer layer) {
    boolean executedState1 = super.executeState1(layer);
    if (executedState1) {
      for(ReferenceField ref : refFields) {
        layer.enterScope(ref.uid, ref.fieldAccess.get(), ref.refUpdater);
      }
    }
    return executedState1;
  }

  protected void terminate(Layer layer) {
    super.terminate(layer);
    for(ReferenceField ref : refFields) {
      layer.exitScope(ref.uid);
    }
  }
}
