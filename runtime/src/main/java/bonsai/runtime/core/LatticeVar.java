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

package bonsai.runtime.core;

// A lattice-based variable must implement two operations over lattice: `join` for adding information and `entail` for asking if a piece of information can be deduced.

public abstract class LatticeVar {
  public abstract void join(Object o);
  public abstract EntailmentResult entail(Object o);
  public abstract LatticeVar bottom();

  public boolean equals (Object other) {
    if (other == null) {
      return false;
    }
    else if (!(other instanceof LatticeVar)) {
      return false;
    }
    else {
      LatticeVar o = (LatticeVar) other;
      return
        this.entail(o) == EntailmentResult.TRUE &&
        o.entail(this) == EntailmentResult.TRUE;
    }
  }
}
