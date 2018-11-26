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
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class FunctionCall extends NAryCall implements Expression
{
  private final Function<ArrayList<Object>, Object> function;
  private ExprResult result;

  public FunctionCall(List<Access> args, Function<ArrayList<Object>, Object> function) {
    super(args);
    this.result = new ExprResult();
    this.function = function;
  }

  public FunctionCall copy() {
    return new FunctionCall(ASTNode.copyList(args), function);
  }

  public void canInstant(Layer layer) {
    super.canInstant(layer);
    result = new ExprResult();
  }

  public ExprResult execute(Layer layer) {
    if (result.isSuspended()) {
      boolean ready = super.executeArgs(layer);
      if (ready) {
        result = new ExprResult(function.apply(argsEval));
      }
    }
    return result;
  }
}
