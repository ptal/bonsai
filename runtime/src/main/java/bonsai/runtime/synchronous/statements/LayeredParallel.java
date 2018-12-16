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
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

/// A parallel statement like `p1 || ... || pN` is stateful in each layer, thus we create one parallel statement for each layer.
/// This class acts as a dipatcher on the parallel statement on the right layer.

public class LayeredParallel extends ASTNode implements Statement
{
  public static int CONJUNCTIVE_PAR = 0;
  public static int DISJUNCTIVE_PAR = 1;

  private final List<Statement> processes;
  private final int kind;
  private final ArrayList<Statement> layeredPar;

  public LayeredParallel(List<Statement> processes, int kind) {
    super();
    this.processes = processes;
    this.kind = kind;
    this.layeredPar = new ArrayList();
    Statement par0 = createPar(processes, kind, 0);
    int layers = par0.countLayers();
    layeredPar.add(par0);
    for(int i = 1; i <= layers; i++) {
      layeredPar.add(createPar(processes, kind, i));
    }
  }

  private Statement createPar(List<Statement> processes, int kind, int layer) {
    Statement layerPar;
    if (kind == CONJUNCTIVE_PAR) {
      layerPar = new ConjunctivePar(processes, layer);
    }
    else if (kind == DISJUNCTIVE_PAR) {
      layerPar = new DisjunctivePar(processes, layer);
    }
    else {
      throw new RuntimeException("`kind` must be either `CONJUNCTIVE_PAR` or `DISJUNCTIVE_PAR`.");
    }
    return layerPar;
  }

  private Statement top() {
    return top();
  }

  public LayeredParallel copy() {
    return new LayeredParallel(ASTNode.copyList(processes), kind);
  }

  public void prepare() {
    for(Statement p : layeredPar) {
      p.prepare();
    }
  }

  public void canInstant(int layersRemaining, Layer layer) {
    layeredPar.get(layersRemaining).canInstant(layersRemaining, layer);
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    return layeredPar.get(layersRemaining).activeQueues(layersRemaining);
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    return layeredPar.get(layersRemaining).endOfInstant(layersRemaining, layer);
  }

  public boolean canTerminate() {
    return top().canTerminate();
  }

  public void abort(Layer layer) {
    top().abort(layer);
  }

  public void suspend(Layer layer) {
    top().suspend(layer);
  }

  public StmtResult execute(int layersRemaining, Layer layer) {
    return layeredPar.get(layersRemaining).execute(layersRemaining, layer);
  }

  public boolean canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    return layeredPar.get(layersRemaining).canWriteOn(layersRemaining, layer, uid, inSurface);
  }

  public int countLayers() {
    return top().countLayers();
  }
}
