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

public class ProcedureCall extends NAryCall
{
  private CompletionCode result;
  private Consumer<ArrayList<Object>> procedure;

  public ProcedureCall(List<Access> args, Consumer<ArrayList<Object>> procedure) {
    super(args);
    this.result = CompletionCode.WAIT;
    this.procedure = procedure;
  }

  public void prepareSubInstant(Environment env, int layerIndex) {
    throw new NoSubLayerException("ProcedureCall.prepareSubInstant");
  }
  public CompletionCode executeSub(Environment env, int layerIndex) {
    throw new NoSubLayerException("ProcedureCall.executeSub");
  }

  public void prepareInstant(Layer layer) {
    result = CompletionCode.WAIT;
    super.prepareInstant(layer);
  }

  public CompletionCode execute(Layer layer) {
    if (result == CompletionCode.WAIT) {
      boolean ready = super.executeArgs(layer);
      if (ready) {
        procedure.accept(argsEval);
        result = CompletionCode.TERMINATE;
      }
    }
    return result;
  }
}
