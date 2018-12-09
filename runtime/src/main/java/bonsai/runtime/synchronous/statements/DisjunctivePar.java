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

public class DisjunctivePar extends ASTNode implements Statement
{
  private final List<Statement> par;
  private ArrayList<LayerData> layersData;
  private int currentLayer;

  class LayerData {
    public ArrayList<Integer> active;
    public ArrayList<StmtResult> results;
    public LayerData() {
      init();
    }

    public void init() {
      active = new ArrayList();
      results = new ArrayList();
      for(int i = 0; i < par.size(); i++) {
        active.add(i);
        results.add(new StmtResult(CompletionCode.WAIT));
      }
    }
  }

  public DisjunctivePar(List<Statement> par) {
    super();
    this.par = par;
    init();
  }

  public DisjunctivePar copy() {
    return new DisjunctivePar(ASTNode.copyList(par));
  }

  private void init() {
    layersData = new ArrayList();
    currentLayer = 0;
  }

  public void prepare() {
    for(Statement p : par) {
      p.prepare();
    }
    init();
  }

  public void canInstant(int layersRemaining, Layer layer) {
    throw new RuntimeException("DisjunctivePar.canInstant: unimplemented.");
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    throw new RuntimeException("DisjunctivePar.activeQueues: unimplemented.");
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    throw new RuntimeException("DisjunctivePar.terminateEmptyQueue: unimplemented.");
  }

  public boolean canTerminate() {
    throw new RuntimeException("DisjunctivePar.canTerminate: unimplemented.");
  }

  public void abort(Layer layer) {
    throw new RuntimeException("DisjunctivePar.abort: unimplemented.");
  }

  public void suspend(Layer layer) {
    throw new RuntimeException("DisjunctivePar.suspend: unimplemented.");
  }

  public StmtResult execute(int layersRemaining, Layer layer) {
    currentLayer = layersRemaining;
    throw new RuntimeException("DisjunctivePar.execute: unimplemented.");
  }

  public boolean canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    throw new RuntimeException("DisjunctivePar.canWriteOn: unimplemented.");
  }

  public int countLayers() {
    int n = 0;
    for (Statement p : par) {
      n = Math.max(n, p.countLayers());
    }
    return n;
  }

  // public void schedule(Schedulable from) {
  //   for (int i = 0; i < par.size(); i++) {
  //     Statement proc = par.get(i);
  //     if (proc == from) {
  //       if (!active.contains(i)) {
  //         active.add(i);
  //       }
  //       break;
  //     }
  //   }
  //   super.schedule(from);
  // }
}
