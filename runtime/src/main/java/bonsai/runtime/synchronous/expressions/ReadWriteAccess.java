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
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.variables.*;

public class ReadWriteAccess extends ASTNode implements Expression
{
  private String name;
  private boolean hasSubscribed;

  public ReadWriteAccess(String name) {
    this.name = name;
    this.hasSubscribed = false;
  }

  public ReadWriteAccess copy() {
    return new ReadWriteAccess(name);
  }

  public ExprResult execute(Environment env) {
    Variable var = env.lookUpVar(name);
    if (var.isReadWritable()) {
      return new ExprResult(var.value());
    }
    else {
      subscribe(env, var);
      return new ExprResult();
    }
  }

  private void subscribe(Environment env, Variable var) {
    if (!hasSubscribed) {
      hasSubscribed = true;
      Event event = new Event(var.uid(), Event.CAN_READWRITE);
      env.subscribe(event, this);
    }
  }

  public void joinRWCounter(Environment env) {
    Variable var = env.lookUpVar(name);
    var.joinReadWrite(env);
  }

  public void meetRWCounter(Environment env) {
    Variable var = env.lookUpVar(name);
    var.meetReadWrite(env);
  }
}
