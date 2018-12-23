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
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;


public class Loop extends ASTNode implements Statement
{

  // We use two bodies: this is only useful when we must "link" the end of the loop with its beginning (when the pause is not at the end, e.g. loop S; pause; S' end).
  // When we reach the end of the loop, we switch between `body` and `surfaceBody`.
  private Statement body;
  private Statement surfaceBody;
  private StmtResult res;
  private boolean canSurface;

  public Loop(Statement body) {
    super();
    this.body = body;
    this.surfaceBody = body.copy();
    init();
  }

  private void init() {
    res = new StmtResult(CompletionCode.WAIT);
    canSurface = false;
  }

  public Loop copy() {
    return new Loop(body.copy());
  }

  public void prepare() {
    body.prepare();
    body.setParent(this);
    init();
  }

  public void canInstant(int layersRemaining, Layer layer) {
    init();
    body.canInstant(layersRemaining, layer);
    if (layersRemaining == 0 && body.canTerminate()) {
      canSurface = true;
      layer.enterLoopSurface();
      surfaceBody.canInstant(layersRemaining, layer);
      layer.exitLoopSurface();
    }
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    return body.activeQueues(layersRemaining);
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    CompletionCode bodyRes = body.endOfInstant(layersRemaining, layer);
    if (layersRemaining == 0 && bodyRes == CompletionCode.TERMINATE) {
      goToBeginningLoopInInstant();
      return CompletionCode.PAUSE;
    }
    else {
      return bodyRes;
    }
  }

  public boolean canTerminate() {
    return res.k == CompletionCode.TERMINATE;
  }

  public void abort(Layer layer) {
    res = new StmtResult(CompletionCode.TERMINATE);
    body.abort(layer);
  }

  public void suspend(Layer layer) {
    body.suspend(layer);
  }

  private void goToBeginningLoopInInstant() {
    Statement tmp = body;
    body = surfaceBody;
    surfaceBody = tmp;
    surfaceBody.prepare();
    canSurface = false;
  }

  public StmtResult execute(int layersRemaining, Layer layer){
    // System.out.println("Loop.execute(" + layersRemaining + ", )");
    if (layersRemaining == 0) {
      if (res.k != CompletionCode.TERMINATE) {
        res.sequence(body.execute(layersRemaining, layer));
        if (res.k == CompletionCode.TERMINATE) {
          goToBeginningLoopInInstant();
          res.sequence(body.execute(layersRemaining, layer));
        }
        else if (!res.k.isInternal() && canSurface) {
          surfaceBody.abort(layer);
        }
      }
      return res;
    }
    else {
      return body.execute(layersRemaining, layer);
    }
  }

  public CanWriteOnResult canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    // System.out.println("Loop.canWriteOn");
    if (layersRemaining == 0) {
      CanWriteOnResult resDepth = body.canWriteOn(layersRemaining, layer, uid, inSurface);
      if (resDepth.canTerminate) {
        resDepth.join(surfaceBody.canWriteOn(layersRemaining, layer, uid, inSurface));
      }
      return resDepth;
    }
    else {
      return body.canWriteOn(layersRemaining, layer, uid, inSurface);
    }
  }

  public int countLayers() { return body.countLayers(); }
}
