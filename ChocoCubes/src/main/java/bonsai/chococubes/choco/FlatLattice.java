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

// `FlatLattice` transforms any type `T` ranging over values into a lattice of the form:
//       Top
//   /  / |   \
// E1 E2 E3 .. EN
//   \ \  |   /
//     Bottom

// The bottom element is represented with an empty optional and top should never happen (assertion).

package bonsai.chococubes.choco;

import java.util.Optional;

public class FlatLattice<T> {

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

  public EntailmentResult entails(FlatLattice<T> other) {
    if (other.isBottom()) {
      return EntailmentResult.True;
    }
    else {
      return this.entails(other.value.get());
    }
  }

  public EntailmentResult entails(T other) {
    if (this.isBottom()) {
      return EntailmentResult.Unknown;
    }
    else {
      T this_value = value.get();
      if (this_value.equals(other)) {
        return EntailmentResult.True;
      }
      else {
        return EntailmentResult.False;
      }
    }
  }

  public void tell(FlatLattice<T> other) {
    if (!other.isBottom()) {
      this.tell(other.value.get());
    }
  }

  public void tell(T other) {
    if (!this.isBottom()) {
      assert this.value.get().equals(other) :
        "Reached TOP element in consistent lattice.";
    }
    this.value = Optional.of(other);
  }
}
