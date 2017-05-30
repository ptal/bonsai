// Copyright 2017 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// `SnapshotST` stores all the single time variables captured in a space statement.
/// These variables are shared among all the branches of the space statement.
/// Note that the branch must have access to these variables only in READ-ONLY (TODO: ensure this property in the bonsai compiler).

package bonsai.runtime.sugarcubes;

import java.util.*;
import bonsai.runtime.core.*;

public class SnapshotST
{
  private HashMap<String, Object> singleTimeVars;

  public SnapshotST() {
    this.singleTimeVars = new HashMap();
  }

  public void saveSingleTimeVar(String uid, Object value) {
    singleTimeVars.put(uid, value);
  }

  public Optional<Object> getSingleTimeValue(String uid) {
    Object val = singleTimeVars.get(uid);
    return Optional.ofNullable(val);
  }
}
