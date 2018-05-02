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

package bonsai.runtime.lattice;

import bonsai.runtime.core.Kleene;

// A lattice-based variable must implement two operations over lattice: `join` for adding information and `entail` for asking if a piece of information can be deduced.

public interface Lattice {

  Lattice join(Object o);
  void join_in_place(Object o);

  // The result of the entailment must reflect the current entailment relation between two objects.
  // We do not take care of the fact that objects can evolve, this is taken care of in the semantics of spacetime.
  // For example, if the lattice is a totally ordered set, `entail` never returns `Kleene.UNKNOWN`.
  Kleene entail(Object o);

  // We do not use the Object method `equals` because we want to provide a default implementation (and keep `Lattice` an interface).
  default boolean eq(Object other) {
    if (other == null) {
      return false;
    }
    else if (!(other instanceof Lattice)) {
      return false;
    }
    else {
      Lattice o = (Lattice) other;
      return
        this.entail(o) == Kleene.TRUE &&
        o.entail(this) == Kleene.TRUE;
    }
  }

  default Kleene strict_entail(Object other) {
    if (other == null) {
      return Kleene.FALSE;
    }
    else if (!(other instanceof Lattice)) {
      return Kleene.FALSE;
    }
    else {
      Lattice o = (Lattice) other;
      return
        Kleene.and(this.entail(o), Kleene.fromBool(this.eq(o)));
    }
  }
}
