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
  protected final Statement body;
  protected StmtResult bodyRes;
  protected StmtResult res;

  public QFUniverse(Statement body) {
    super();
    this.body = body;
    prepare();
  }

  public QFUniverse copy() {
    // throw new CannotCopyException("QFUniverse");
    return new QFUniverse(body.copy());
  }

  public void prepare() {
    prepareInstant();
  }

  public void prepareInstant() {
    res = new StmtResult(CompletionCode.PAUSE_DOWN);
    bodyRes = new StmtResult(CompletionCode.WAIT);
  }

  public void canInstant(int layersRemaining, Layer layer) {
    if (layersRemaining == 0) {
      prepareInstant();
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

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    checkNonTerminatedEOI("universe p end", res.k);
    if(layersRemaining == 0) {
      return res.k;
    }
    else {
      return body.endOfInstant(layersRemaining - 1, layer);
    }
  }

  public boolean canTerminate() {
    return body.canTerminate();
  }

  public void abort(Layer layer) {
    res = new StmtResult(CompletionCode.TERMINATE);
    bodyRes = new StmtResult(CompletionCode.TERMINATE);
  }

  public void suspend(Layer layer) {}

  public StmtResult execute(int layersRemaining, Layer layer){
    if (layersRemaining == 0) {
      // Promote the completion code of the body.
      switch(bodyRes.k) {
        case PAUSE_UP: res.k = CompletionCode.PAUSE; break;
        case STOP: res.k = CompletionCode.STOP; break;
        case TERMINATE: res.k = CompletionCode.TERMINATE; break;
      }
      return res;
    }
    else {
      bodyRes = body.execute(layersRemaining - 1, layer);
      return bodyRes;
    }
  }

  public CanWriteOnResult canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    if (layersRemaining == 0) {
      return new CanWriteOnResult(canTerminate(), false);
    }
    else {
      return body.canWriteOn(layersRemaining - 1, layer, uid, inSurface);
    }
  }

  public int countLayers() { return 1 + body.countLayers(); }
}
