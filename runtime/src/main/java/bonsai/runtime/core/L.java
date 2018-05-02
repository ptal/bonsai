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

// `L<T>` transforms any type `T` ranging over values `E1...EN` into a lattice of the form:
//       Top
//   /  / |   \
// E1 E2 E3 .. EN
//   \ \  |   /
//     Bottom

// The bottom element is represented with an empty optional and top should never happen (exception otherwise).

// The entailment and join operation are dynamically overload so `x |= y` and `x <- y` are defined such that `y` can be of type `L<T>` or `T`.

package bonsai.runtime.core;

import bonsai.runtime.lattice.*;
import java.util.Optional;

public class L<T> implements Lattice, Resettable<L<T>>, Copy<L<T>>
{
  protected Optional<T> value;

  public L(T value) {
    this.value = Optional.of(value);
  }

  public L() {
    this.value = Optional.empty();
  }

  public L<T> bottom() {
    return new L();
  }

  public boolean isBottom() {
    return !value.isPresent();
  }

  public void reset(L<T> o) {
    this.value = o.value;
  }

  public L<T> copy() {
    if (isBottom()) {
      return bottom();
    }
    else {
      Copy v = Cast.toCopy("<anon> in L.copy", value.get());
      return new L(v.copy());
    }
  }

  public Kleene entail(Object obj) {
    L<T> other = flatLatticeOf(obj);
    if (other.isBottom()) {
      return Kleene.TRUE;
    }
    T other_inner = other.value.get();
    return entail_inner(other_inner);
  }

  private Kleene entail_inner(T other) {
    if (this.isBottom()) {
      return Kleene.UNKNOWN;
    }
    else {
      T self = value.get();
      assertSameInnerTypes(self, other);
      if (self.equals(other)) {
        return Kleene.TRUE;
      }
      else {
        return Kleene.FALSE;
      }
    }
  }

  public L<T> join(Object value) {
    throw new UnsupportedOperationException(
      "Join is currently not defined for `L<T>`.");
  }

  public void join_in_place(Object obj) {
    L<T> other = flatLatticeOf(obj);
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
          "Reached TOP element in flat lattice due to `"
          + self + "` join `" + other + "`");
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

  private L<T> flatLatticeOf(Object obj) {
    assert obj != null;
    if (obj instanceof L) {
      return (L) obj;
    }
    else {
      return new L(obj);
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

  public T unwrap() {
    return value.get();
  }
}
