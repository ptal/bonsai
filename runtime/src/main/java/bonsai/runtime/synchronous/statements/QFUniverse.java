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
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.exceptions.*;
import bonsai.runtime.synchronous.env.*;

public class QFUniverse extends ASTNode implements Statement
{
  private final Statement body;
  private StmtResult bodyRes;
  private CompletionCode k;

  public QFUniverse(Statement body) {
    super();
    this.body = body;
    prepare();
  }

  public QFUniverse copy() {
    throw new CannotCopyException("QFUniverse");
  }

  public void prepare() {
    k = CompletionCode.PAUSE_DOWN;
    bodyRes = new StmtResult(CompletionCode.WAIT);
  }

  public void canInstant(int layersRemaining, Layer layer) {
    if (layersRemaining == 0) {
      prepare();
    }
    else {
      body.canInstant(layersRemaining - 1, layer);
    }
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    if (layersRemaining > 1) {
      return body.activeQueues(layersRemaining - 1);
    }
    else {
      return new HashSet();
    }
  }

  public boolean canTerminate() {
    return body.canTerminate();
  }

  public void abort(Layer layer) {
    k = CompletionCode.TERMINATE;
    bodyRes = new StmtResult(CompletionCode.TERMINATE);
  }

  public void suspend(Layer layer) {}

  public StmtResult execute(int layersRemaining, Layer layer){
    if (layersRemaining == 0) {
      // Promote the completion code of the body.
      switch(bodyRes.k) {
        case PAUSE_UP: k = CompletionCode.PAUSE; break;
        case STOP: k = CompletionCode.STOP; break;
        case TERMINATE: k = CompletionCode.TERMINATE; break;
      }
      return new StmtResult(k);
    }
    else {
      bodyRes = body.execute(layersRemaining - 1, layer);
      return bodyRes;
    }
  }

  public boolean canWriteOn(int layersRemaining, String uid, boolean inSurface) {
    if (layersRemaining == 0) {
      return false;
    }
    else {
      return body.canWriteOn(layersRemaining - 1, uid, inSurface);
    }
  }

  public int countLayers() { return 1 + body.countLayers(); }
}
