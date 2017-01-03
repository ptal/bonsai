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

// `FlatLattice` transforms any type `T` ranging over values `E1...EN` into a lattice of the form:
//       Top
//   /  / |   \
// E1 E2 E3 .. EN
//   \ \  |   /
//     Bottom

// The bottom element is represented with an empty optional and top should never happen (exception otherwise).

// The entailment and join operation are dynamically overload so `x |= y` and `x <- y` are defined such that `y` can be of type `FlatLattice<T>` or `T`.

package bonsai.runtime.core;

import java.util.Optional;

public class FlatLattice<T> extends LatticeVar {

  protected Optional<T> value;

  public FlatLattice(T value) {
    this.value = Optional.of(value);
  }

  public FlatLattice() {
    this.value = Optional.empty();
  }

  public static <E> FlatLattice<E> bottom() {
    return new FlatLattice<E>();
  }

  public boolean isBottom() {
    return !value.isPresent();
  }

  public EntailmentResult entail(Object obj) {
    FlatLattice<T> other = flatLatticeOf(obj);
    if (other.isBottom()) {
      return EntailmentResult.TRUE;
    }
    T other_inner = other.value.get();
    return entail_inner(other_inner);
  }

  private EntailmentResult entail_inner(T other) {
    if (this.isBottom()) {
      return EntailmentResult.UNKNOWN;
    }
    else {
      T self = value.get();
      assertSameInnerTypes(self, other);
      if (self.equals(other)) {
        return EntailmentResult.TRUE;
      }
      else {
        return EntailmentResult.FALSE;
      }
    }
  }

  public void join(Object obj) {
    FlatLattice<T> other = flatLatticeOf(obj);
    if (!other.isBottom()) {
      T other_inner = other.value.get();
      join_inner(other_inner);
    }
  }

  public void join_inner(T other) {
    assert other != null;
    if (this.isBottom()) {
      this.value = Optional.of(other);
    }
    else {
      T self = this.value.get();
      assertSameInnerTypes(self, other);
      if(!self.equals(other)) {
        throw new RuntimeException(
          "Reached TOP element in flat lattice.");
      }
    }
  }

  private void assertSameInnerTypes(T self, Object other) {
    assert self != null && other != null;
    if (!self.getClass().isInstance(other)) {
      throw new RuntimeException(
        "Undefined entailment between `" +
        this.getClass().getCanonicalName() +
        "` and `" +
        other.getClass().getCanonicalName() + "`");
    }
  }

  private FlatLattice<T> flatLatticeOf(Object obj) {
    assert obj != null;
    if (obj instanceof FlatLattice) {
      return (FlatLattice) obj;
    }
    else {
      return new FlatLattice(obj);
    }
  }

  public String toString() {
    if (isBottom()) {
      return "bottom";
    }
    else {
      return value.get().toString();
    }
  }
}
