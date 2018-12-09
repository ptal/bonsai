// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// `Space` represents the spacetime statement `space b end`.
/// The fields `branch` is the code of the branch `b` and `singleTimeClosure` contains the UIDs of the `single_time` variables captured in this branch.

package bonsai.runtime.synchronous.statements;

import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.exceptions.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.variables.*;
import bonsai.runtime.synchronous.search.*;

public class SpaceStmt extends ASTNode implements Statement
{
  private final ArrayList<String> capturedUIDs;
  private final Statement branch;

  public SpaceStmt(ArrayList<String> capturedUIDs, Statement branch) {
    super();
    this.capturedUIDs = capturedUIDs;
    this.branch = branch;
  }

  public SpaceStmt copy() {
    throw new CannotCopyException("SpaceStmt");
  }

  public void prepare() {}

  public void canInstant(int layersRemaining, Layer layer) {
    checkNoSubLayer(layersRemaining, "SpaceStmt.canInstant");
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    return new HashSet();
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    throwNonTerminatedEOI("space p end");
    return CompletionCode.TERMINATE;
  }

  public boolean canTerminate() { return true; }

  public void suspend(Layer layer) {}

  public void abort(Layer layer) {}

  public StmtResult execute(int layersRemaining, Layer layer) {
    checkNoSubLayer(layersRemaining, "SpaceStmt.execute");
    HashMap<String, Variable> projection = layer.project(capturedUIDs);
    CapturedSpace capturedSpace = new CapturedSpace(projection);
    BranchAlgebra ba = BranchAlgebra.spaceBranch(branch, capturedSpace);
    return new StmtResult(CompletionCode.TERMINATE, layer.currentQueue(), ba);
  }

  public boolean canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    checkNoSubLayer(layersRemaining, "SpaceStmt.canWriteOn");
    return false;
  }

  public int countLayers() { return 0; }
}
