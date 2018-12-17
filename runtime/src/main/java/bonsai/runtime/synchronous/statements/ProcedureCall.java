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
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class ProcedureCall extends NAryCall implements Statement
{
  private final Consumer<ArrayList<Object>> procedure;
  private CompletionCode k;

  public ProcedureCall(List<Access> args, Consumer<ArrayList<Object>> procedure) {
    super(args);
    this.procedure = procedure;
    prepare();
  }

  public ProcedureCall copy() {
    return new ProcedureCall(ASTNode.copyList(args), procedure);
  }

  public void prepare() {
    k = CompletionCode.WAIT;
  }

  public void canInstant(int layersRemaining, Layer layer) {
    checkNoSubLayer(layersRemaining, "ProcedureCall.canInstant");
    super.canInstant(layer);
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    return new HashSet();
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    throwNonTerminatedEOI("call");
    return CompletionCode.TERMINATE;
  }

  public boolean canTerminate() {
    return true;
  }

  public void abort(Layer layer) {
    super.terminate(layer);
  }

  public void suspend(Layer layer) {
    super.terminate(layer);
  }

  public StmtResult execute(int layersRemaining, Layer layer) {
    checkNoSubLayer(layersRemaining, "ProcedureCall.execute");
    if (k == CompletionCode.WAIT) {
      boolean ready = super.executeArgs(layer);
      if (ready) {
        procedure.accept(argsEval);
        super.terminate(layer);
        k = CompletionCode.TERMINATE;
      }
    }
    return new StmtResult(k);
  }

  public CanWriteOnResult canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    checkNoSubLayer(layersRemaining, "ProcedureCall.canWriteOn");
    // System.out.println("ProcedureCall.canWriteOn: " + uid);
    return new CanWriteOnResult(true, super.canWriteOn(uid));
  }

  public int countLayers() { return 0; }
}
