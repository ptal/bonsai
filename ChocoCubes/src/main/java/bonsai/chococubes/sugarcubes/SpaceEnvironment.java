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
import inria.meije.rc.sugarcubes.implementation.*;
import inria.meije.rc.sugarcubes.*;

public class SpaceEnvironment extends Clock {
  private HashMap<String, SpacetimeVar> vars;

  // private ArrayList<SpaceBranch> spaceBranches;
  // private ArrayList<Snapshot> spaceQueue;

  public SpaceEnvironment(ClockIdentifier clockID,
    InternalIdentifiers anInternalIdentifierGenerator,
    Program body)
  {
    super(clockID, anInternalIdentifierGenerator, body);
    vars = new HashMap();

    // spaceBranches = new ArrayList();
    // spaceQueue = new ArrayList();
  }

  // Big step transition.
  public void newInstant() {
    super.newInstant();
  }

  public void declareVar(String name, SpacetimeVar v) {
    vars.put(name, v);
  }

  public void freeVar(String name) {
    vars.remove(name);
  }

  public LatticeVar var(String name) {
    return vars.get(name).latticeValue();
  }
}
