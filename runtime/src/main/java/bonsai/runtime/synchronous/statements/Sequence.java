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
  private int reachable;

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
    reachable = 0;
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
      res = new StmtResult(res.k);
      reachable = pc - 1;
      reachableSubsequence((p) -> {reachable++; p.canInstant(layersRemaining, layer);});
    }
    else {
      current().canInstant(layersRemaining, layer);
    }
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    if (layersRemaining == 0) {
      return new HashSet();
    }
    else {
      return current().activeQueues(layersRemaining);
    }
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    // This can only happens if the sequence is at top-level.
    if(layersRemaining == 0) {
      checkNonTerminatedEOI("p;q", res.k);
      CompletionCode k = current().endOfInstant(layersRemaining, layer);
      if (k == CompletionCode.TERMINATE) {
        pc++;
        if (pc == seq.size()) {
          res.k = CompletionCode.TERMINATE;
        }
        else {
          res.k = current().endOfInstant(layersRemaining, layer);
          for(int i = pc; i <= reachable; i++) {
            seq.get(i).abort(layer);
          }
        }
      }
      return res.k;
    }
    else {
      return current().endOfInstant(layersRemaining, layer);
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
        res.sequence(current().execute(layersRemaining, layer));
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

  public CanWriteOnResult canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    if (layersRemaining == 0) {
      CanWriteOnResult canRes = new CanWriteOnResult(true, false);
      System.out.println("Sequence.canWriteOn: " + uid);
      if (pc < seq.size()) {
        canRes = current().canWriteOn(layersRemaining, layer, uid, inSurface);
        for(int i=pc+1; i < seq.size() && canRes.canTerminate && !canRes.canWrite; i++) {
          Statement p = seq.get(i);
          canRes = canRes.join(p.canWriteOn(layersRemaining, layer, uid, false));
        }
      }
      return canRes;
    }
    else {
      return current().canWriteOn(layersRemaining, layer, uid, inSurface);
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
