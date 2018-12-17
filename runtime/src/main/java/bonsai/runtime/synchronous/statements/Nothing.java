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
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class Nothing extends ASTNode implements Statement
{
  public Nothing() {
    super();
  }

  public Nothing copy() {
    return new Nothing();
  }

  public void prepare() {}

  public void canInstant(int layersRemaining, Layer layer) {
    checkNoSubLayer(layersRemaining, "Nothing.canInstant");
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    return new HashSet();
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    throwNonTerminatedEOI("nothing");
    return CompletionCode.TERMINATE;
  }

  public boolean canTerminate() {
    return true;
  }

  public void abort(Layer layer) {}
  public void suspend(Layer layer) {}

  public StmtResult execute(int layersRemaining, Layer layer){
    checkNoSubLayer(layersRemaining, "Nothing.execute");
    return new StmtResult(CompletionCode.TERMINATE);
  }

  public CanWriteOnResult canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    checkNoSubLayer(layersRemaining, "Nothing.canWriteOn");
    return new CanWriteOnResult(true, false);
  }

  public int countLayers() { return 0; }
}
