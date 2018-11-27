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

public class ConjunctivePar extends ASTNode implements Statement
{
  private final List<Statement> par;

  public ConjunctivePar(List<Statement> par) {
    super();
    this.par = par;
    init();
  }

  public ConjunctivePar copy() {
    return new ConjunctivePar(ASTNode.copyList(par));
  }

  private void init() {}

  public void prepare() {
    for(Statement p : par) {
      p.prepare();
    }
    init();
  }

  public void canInstant(int layersRemaining, Layer layer) {
    throw new RuntimeException("ConjunctivePar.canInstant: unimplemented.");
  }

  public boolean canTerminate() {
    throw new RuntimeException("ConjunctivePar.canTerminate: unimplemented.");
  }

  public void abort(Layer layer) {
    throw new RuntimeException("ConjunctivePar.abort: unimplemented.");
  }

  public void suspend(Layer layer) {
    throw new RuntimeException("ConjunctivePar.suspend: unimplemented.");
  }

  public StmtResult execute(int layersRemaining, Layer layer) {
    throw new RuntimeException("ConjunctivePar.execute: unimplemented.");
  }

  public boolean canWriteOn(int layersRemaining, String uid, boolean inSurface) {
    throw new RuntimeException("ConjunctivePar.canWriteOn: unimplemented.");
  }

  public int countLayers() {
    throw new RuntimeException("ConjunctivePar.countLayers: unimplemented.");
  }
}
