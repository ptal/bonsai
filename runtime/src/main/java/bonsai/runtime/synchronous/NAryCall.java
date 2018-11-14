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

package bonsai.runtime.synchronous;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.expressions.*;
import bonsai.runtime.synchronous.env.*;

public abstract class NAryCall extends ASTNode
{
  protected final List<Access> args;
  protected ArrayList<Object> argsEval;

  public NAryCall(List<Access> args) {
    this.args = args;
    this.argsEval = new ArrayList(args.size());
  }

  protected void canInstant(Layer layer) {
    argsEval.clear();
    for(int i=0; i < args.size(); i++) {
      argsEval.add(null);
    }
    for (Access access : args) {
      access.canInstant(layer);
    }
  }

  public void terminate(Layer layer) {
    for (Access access : args) {
      access.terminate(layer);
    }
  }

  protected boolean executeArgs(Layer layer) {
    boolean ready = true;
    for(int i=0; i < args.size(); i++) {
      ExprResult res = args.get(i).execute(layer);
      if (res.isSuspended()) {
        ready = false;
      }
      else {
        argsEval.set(i, res.unwrap());
      }
    }
    return ready;
  }

  public boolean canWriteOn(String uid) {
    for(Access arg: args) {
      if (arg.canWriteOn(uid)) {
        return true;
      }
    }
    return false;
  }
}
