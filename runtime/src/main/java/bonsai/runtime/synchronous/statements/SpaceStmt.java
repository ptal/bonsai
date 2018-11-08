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
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;

public class SpaceStmt extends ASTNode implements Program
{
  private ArrayList<String> capturedUIDs;
  private Program branch;

  public SpaceStmt(ArrayList<String> capturedUIDs, Program branch) {
    super();
    this.capturedUIDs = capturedUIDs;
    this.branch = branch;
  }

  void throwSubError(String method) {
    throw new RuntimeException("SpaceStmt." + method +
      ": should be executed before the sub-layer if it is reachable.");
  }
  public void prepareSubInstant(Environment env, int layerIndex) {
    throwSubError("prepareSubInstant");
  }
  public CompletionCode executeSub(Environment env, int layerIndex) {
    throwSubError("executeSub");
    return null;
  }

  public void prepareInstant(Layer env) {}
  public CompletionCode execute(Layer env) {
    return CompletionCode.TERMINATE;
  }

  public CanResult canWriteOn(String uid, boolean inSurface) {
    return new CanResult(true, false);
  }

  public void meetRWCounter(Layer env) {}

  public ArrayList<String> capturedUIDs() {
    return capturedUIDs;
  }

  public Program branch() {
    return branch;
  }
}
