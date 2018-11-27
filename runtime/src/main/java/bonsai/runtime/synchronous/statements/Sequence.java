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
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class Sequence extends ASTNode implements Statement
{
  private final List<Statement> seq;
  private int pc; // program counter
  private StmtResult res;

  public Sequence(List<Statement> seq) {
    super();
    this.seq = seq;
    init();
  }

  public Sequence copy() {
    return new Sequence(ASTNode.copyList(seq));
  }

  private Statement current() {
    return seq.get(pc);
  }

  private void init() {
    pc = 0;
    res = new StmtResult(CompletionCode.TERMINATE);
  }

  public void prepare() {
    for(Statement p : seq) {
      p.prepare();
    }
    init();
  }

  private boolean reachableSubsequence(Consumer<Statement> f) {
    boolean canTerminate = true;
    for(int i=pc; i < seq.size() && canTerminate; i++) {
      Statement p = seq.get(i);
      f.accept(p);
      canTerminate = p.canTerminate();
    }
    return canTerminate;
  }

  public void canInstant(int layersRemaining, Layer layer) {
    if (layersRemaining == 0) {
      reachableSubsequence((p) -> p.canInstant(layersRemaining, layer));
    }
    else {
      current().canInstant(layersRemaining, layer);
    }
  }

  public boolean canTerminate() {
    return reachableSubsequence((p) -> {});
  }

  public void abort(Layer layer) {
    reachableSubsequence((p) -> p.abort(layer));
  }

  public void suspend(Layer layer) {
    reachableSubsequence((p) -> p.suspend(layer));
  }

  public StmtResult execute(int layersRemaining, Layer layer) {
    if (layersRemaining == 0) {
      while (pc < seq.size()) {
        res = current().execute(layersRemaining, layer);
        if (res.k == CompletionCode.TERMINATE) {
          pc++;
        }
        else {
          break;
        }
      }
      return res;
    }
    else {
      return current().execute(layersRemaining, layer);
    }
  }

  public boolean canWriteOn(int layersRemaining, String uid, boolean inSurface) {
    if (layersRemaining == 0) {
      boolean canTerminate = true;
      boolean canWrite = false;
      if (pc < seq.size()) {
        canWrite = current().canWriteOn(layersRemaining, uid, inSurface);
        for(int i=pc; i < seq.size() && canTerminate && !canWrite; i++) {
          Statement p = seq.get(i);
          canWrite = p.canWriteOn(layersRemaining, uid, false);
          canTerminate = p.canTerminate();
        }
      }
      return canWrite;
    }
    else {
      return current().canWriteOn(layersRemaining, uid, inSurface);
    }
  }

  public int countLayers() {
    int layers = 0;
    for(Statement p : seq) {
      layers = Math.max(p.countLayers(), layers);
    }
    return layers;
  }
}
