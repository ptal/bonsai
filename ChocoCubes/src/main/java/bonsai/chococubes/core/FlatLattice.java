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

// The bottom element is represented with an empty optional and top should never happen (assertion).

package bonsai.chococubes.core;

import java.util.Optional;

public class FlatLattice<T> extends LatticeVar {

  private Optional<T> value;

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

  private FlatLattice<T> castTo(Object obj) {
    assert obj != null && this.getClass().isInstance(obj);
    return (FlatLattice<T>) obj;
  }

  public EntailmentResult entail(Object obj) {
    FlatLattice<T> other = this.castTo(obj);
    if (other.isBottom()) {
      return EntailmentResult.TRUE;
    }
    else {
      return this.entail_inner(other.value.get());
    }
  }

  public EntailmentResult entail_inner(T other) {
    assert other != null;
    if (this.isBottom()) {
      return EntailmentResult.UNKNOWN;
    }
    else {
      T self = value.get();
      if (self.equals(other)) {
        return EntailmentResult.TRUE;
      }
      else {
        return EntailmentResult.FALSE;
      }
    }
  }

  public void join(Object obj) {
    FlatLattice<T> other = this.castTo(obj);
    if (!other.isBottom()) {
      T inner = other.value.get();
      join_inner(inner);
    }
  }

  public void join_inner(T other) {
    assert other != null;
    if(!this.isBottom() && this.value.get().equals(other)) {
      throw new RuntimeException(
        "Reached TOP element in flat lattice.");
    }
    this.value = Optional.of(other);
  }
}
