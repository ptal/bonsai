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
import java.util.stream.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.exceptions.*;

public abstract class ASTNode implements Schedulable
{
  protected Schedulable parent;

  public ASTNode() {
    parent = null;
  }

  public void setParent(Schedulable parent) {
    this.parent = parent;
  }

  public void schedule(Schedulable from) {
    parent.schedule(from);
  }

  protected void checkNoSubLayer(int layersRemaining, String from) {
    if (layersRemaining > 0) {
      hasNoSubLayer(from);
    }
  }

  protected void hasNoSubLayer(String from) {
    throw new NoSubLayerException(from);
  }

  protected static <T extends Copy> List<T> copyList(List<T> nodes) {
    return nodes.stream().map(v -> (T)v.copy()).collect(Collectors.toList());
  }

  protected void checkNonTerminatedEOI(String nameStmt, CompletionCode k) {
    if (k == CompletionCode.TERMINATE) {
      throwNonTerminatedEOI(nameStmt);
    }
  }

  protected void throwNonTerminatedEOI(String nameStmt) throws RuntimeException {
    throw new RuntimeException("[BUG] `" + nameStmt + "` should not be active at the end of instant.");
  }

  protected void checkExpressionStateEOI(String nameStmt, boolean state) {
    if (state) {
      throw new RuntimeException("[BUG] `" + nameStmt + "` is still blocked in an instantaneous expression at the end of instant.");
    }
  }
}
