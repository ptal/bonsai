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

/// Snapshot of the labels of the world line variables.
/// This snapshot is shared between all the children nodes, and therefore, the `restore` method of must fulfil the `Shared label property`.
/// This property ensures that a label can be restored an arbitrary number of time and is not modified.

package bonsai.runtime.sugarcubes;

import java.util.*;
import bonsai.runtime.core.*;

public class SnapshotWL
{
  private HashMap<String, Object> labelsWL;

  public SnapshotWL() {
    this.labelsWL = new HashMap();
  }

  public void saveWorldLineVar(String uid, Stream stream) {
    labelsWL.put(uid, stream.label());
  }

  public void restoreWorldLineVar(String uid, Stream stream) {
    stream.restore(labelsWL.get(uid));
  }
}
