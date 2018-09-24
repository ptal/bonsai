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

public class WriteAccess extends ASTNode implements Expression
{
  private String name;

  public WriteAccess(String name) {
    this.name = name;
  }

  public WriteAccess copy() {
    return new WriteAccess(name);
  }

  // A write access is always possible.
  public ExprResult execute(Environment env) {
    Variable var = env.lookUpVar(name);
    return new ExprResult(var.value());
  }

  public void joinRWCounter(Environment env) {
    Variable var = env.lookUpVar(name);
    var.rw().write += 1;
  }
  public void meetRWCounter(Environment env) {
    Variable var = env.lookUpVar(name);
    var.rw().write -= 1;
  }
}
