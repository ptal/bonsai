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
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.variables.*;

public class ReadAccess extends Access
{
  private boolean hasSubscribed;

  public ReadAccess(String uid) {
    super(uid);
    this.hasSubscribed = false;
  }

  public void canInstant(Layer layer) {
    super.canInstant(layer);
    Variable var = layer.lookUpVar(uid);
    var.joinRead(layer);
  }

  public void terminate(Layer layer) {
    Variable var = layer.lookUpVar(uid);
    var.meetRead(layer);
  }

  public ExprResult execute(Layer layer) {
    Variable var = layer.lookUpVar(uid);
    if (var.isReadable()) {
      return new ExprResult(var.value());
    }
    else {
      subscribe(layer, Event.CAN_READ);
      return new ExprResult();
    }
  }

  public boolean canWriteOn(String uid) {
    return false;
  }
}
