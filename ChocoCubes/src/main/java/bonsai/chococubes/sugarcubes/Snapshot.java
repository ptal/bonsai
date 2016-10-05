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

package bonsai.chococubes.sugarcubes;

import java.util.*;
import bonsai.chococubes.core.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class Snapshot
{
  private HashMap<String, Object> singleTimeVars;
  private HashMap<String, Object> worldLineVars;
  private int branchIndex;

  public Snapshot(int branchIndex) {
    this.singleTimeVars = new HashMap();
    this.worldLineVars = new HashMap();
    this.branchIndex = branchIndex;
  }

  public int branch() {
    return branchIndex;
  }

  public void saveWorldLineVar(String name, Object value) {
    assertClass(name, value, Restorable.class, "world_line");
    Restorable var = (Restorable) value;
    worldLineVars.put(name, var.label());
  }

  public void saveSingleTimeVar(String name, Object value) {
    singleTimeVars.put(name, value);
  }

  public void restoreWorldLineVar(String name, Object value) {
    assertClass(name, value, Restorable.class, "world_line");
    Restorable var = (Restorable) value;
    var.restore(worldLineVars.get(name));
  }

  public Optional<Object> getSingleTimeValue(String name) {
    Object val = singleTimeVars.get(name);
    return Optional.ofNullable(val);
  }

  private void assertClass(String name, Object value,
    Class expectedTy, String spacetime)
  {
    if (!expectedTy.isInstance(value)) {
      throw new RuntimeException(
        "Variable `" + name + "` does not implement `" + expectedTy.getCanonicalName() +
        "` which is required for variables in the `" + spacetime + "` spacetime.");
    }
  }
}
