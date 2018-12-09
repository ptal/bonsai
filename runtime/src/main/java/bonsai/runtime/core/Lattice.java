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

// A lattice-based variable must implement three operations over lattice:
// * `join` for adding a piece of information according to the order of the lattice.
// * `meet` for removing a piece of information according to the order of the lattice.
// * `entails` for asking if a piece of information can be deduced from the current element.

public interface Lattice
{
  Lattice bottom();
  Lattice top();

  Lattice join(Object o);
  void join_in_place(Object o);

  Lattice meet(Object o);
  void meet_in_place(Object o);

  // The result of the entailment must reflect the current entailment relation between two objects.
  // We do not take care of the fact that objects can evolve, this is taken care of in the semantics of spacetime.
  // For example, if the lattice is a totally ordered set, `entails` never returns `Kleene.UNKNOWN`.
  // The following relation must hold:
  //   * a.entails(b) == TRUE => b.entails(a) != UNKNOWN
  //   * a.entails(b) == FALSE => b.entails(a) == TRUE
  //   * a.entails(b) == UNKNOWN => b.entails(a) == UNKNOWN
  Kleene entails(Object o);

  // NOTE: You should override the method `equals`, possibly using the following default static method `equals_default`.
  // The only way to ensure that `equals` is implemented in sub-classes would be to make `Lattice` an abstract class.

  // (a.entails(b) == TRUE /\ b.entails(a) == TRUE) => a.equals(b) == TRUE
  static boolean equals_default(Lattice l, Object other) {
    if (other == null) {
      return false;
    }
    else if (!(other instanceof Lattice)) {
      return false;
    }
    else {
      Lattice o = (Lattice) other;
      return
        l.entails(o) == Kleene.TRUE &&
        o.entails(l) == Kleene.TRUE;
    }
  }

  // Written `a |< b` in spacetime.
  // a.strict_entail(b) == TRUE => a.entails(b) = TRUE /\ !(a.equals(b))
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
        Kleene.and(this.entails(o), Kleene.fromBool(!equals_default(this,o)));
    }
  }
}
