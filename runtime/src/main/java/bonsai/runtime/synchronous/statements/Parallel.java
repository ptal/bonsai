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

public abstract class Parallel extends ASTNode implements Statement
{
  protected final List<Statement> par;
  protected final int layerIdx;

  protected ArrayList<StmtResult> results;
  // We track the index of the active processes.
  // A process with status WAIT, can become active if it is rescheduled.
  protected ArrayList<Integer> active;

  public Parallel(List<Statement> par, int layerIdx) {
    super();
    this.par = par;
    this.layerIdx = layerIdx;
    init();
  }

  private CompletionCode mergeK() {
    return results.stream()
      .map(r -> r.k)
      .reduce(CompletionCode.TERMINATE, CompletionCode::merge);
  }

  protected abstract StmtResult mergeRes();

  private void init() {
    results = new ArrayList();
    active = new ArrayList();
    for(int i = 0; i < par.size(); i++) {
      active.add(i);
      results.add(new StmtResult(CompletionCode.WAIT));
    }
  }

  public void prepare() {
    for(Statement p : par) {
      p.prepare();
      p.setParent(this);
    }
    init();
  }

  private void activeProcesses(BiConsumer<Integer, Statement> f) {
    for(Integer i : active) {
      Statement p = par.get(i);
      f.accept(i, p);
    }
  }

  public void canInstant(int layersRemaining, Layer layer) {
    active.clear();
    for (int i = 0; i < par.size(); i++) {
      if (results.get(i).k != CompletionCode.TERMINATE) {
        active.add(i);
      }
    }
    activeProcesses((i,s) -> s.canInstant(layersRemaining, layer));
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    HashSet<String> res = new HashSet();
    activeProcesses((i,s) -> {res.addAll(s.activeQueues(layersRemaining));});
    return res;
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    activeProcesses((i,s) -> {
      CompletionCode k = s.endOfInstant(layersRemaining, layer);
      results.get(i).k = k;
    });
    return mergeK();
  }

  private class WrapB {
    public boolean val;
    WrapB(boolean val) { this.val = val; }
  }

  public boolean canTerminate() {
    WrapB canTerm = new WrapB(true);
    activeProcesses((i,s) -> canTerm.val = canTerm.val && s.canTerminate());
    return canTerm.val;
  }

  public void abort(Layer layer) {
    activeProcesses((i,s) -> s.abort(layer));
  }

  public void suspend(Layer layer) {
    activeProcesses((i,s) -> s.suspend(layer));
  }

  public void reduceActive() {
    for(int i = 0; i < par.size(); i++) {
      if (!results.get(i).k.isInternal()) {
        active.remove(new Integer(i));
      }
    }
  }

  public StmtResult execute(int layersRemaining, Layer layer) {
    activeProcesses((i,s) -> results.set(i, s.execute(layersRemaining, layer)));
    reduceActive();
    StmtResult res = mergeRes();
    // System.out.println("Result of parallel : " + res.k);
    return res;
  }

  public CanWriteOnResult canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    CanWriteOnResult res = new CanWriteOnResult(true, false);
    activeProcesses((i,s) -> res.join(s.canWriteOn(layersRemaining, layer, uid, inSurface)));
    return res;
  }

  public int countLayers() {
    int n = 0;
    for (Statement p : par) {
      n = Math.max(n, p.countLayers());
    }
    return n;
  }

  public void schedule(Schedulable from) {
    // System.out.println("Parallel.schedule");
    for (int i = 0; i < par.size(); i++) {
      Statement proc = par.get(i);
      if (proc == from) {
        // System.out.println("Detected from schedulable " + i);
        if (!active.contains(i)) {
          // System.out.println("Wake up process " + i);
          active.add(i);
        }
        break;
      }
    }
    super.schedule(from);
  }
}
