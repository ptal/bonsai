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
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.variables.*;

public class WriteAccess extends ASTNode implements Expression
{
  private String uid;

  public WriteAccess(String uid) {
    this.uid = uid;
  }

  public void prepareInstant(Layer env) {
    Variable var = env.lookUpVar(uid);
    var.joinWrite(env);
  }

  // A write access is always possible.
  public ExprResult execute(Layer env) {
    Variable var = env.lookUpVar(uid);
    return new ExprResult(var.value());
  }

  public CanResult canWriteOn(String uid, boolean inSurface) {
    return new CanResult(true, uid == this.uid);
  }

  public void meetRWCounter(Layer env) {
    Variable var = env.lookUpVar(uid);
    var.meetWrite(env);
  }
}
