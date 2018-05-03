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

// `L<T>` transforms any type `T` ranging over values `E1...EN` into a lattice of the form (called flat lattice):
//       Top
//   /  / |   \
// E1 E2 E3 .. EN
//   \ \  |   /
//     Bottom

// The bottom and top elements are represented with `LKind.BOT` and `LKind.TOP` while all the other elements have the kind `LKind.INNER`.

// The entailment and join operation are defined such that, in `x |= y` and `x <- y`, `y` can be of type `L<T>` or `T`.

// NOTE: The methods `bottom`, `top` and `inner` are not static because they are factory methods that can be overriden in sub-classes (see for example the class `W`).

package bonsai.runtime.lattices;

import bonsai.runtime.core.*;

// `T` must override `equals`.
public class L<T> implements Lattice, Copy<L<T>>
{
  enum LKind {
    BOT, TOP, INNER
  };

  protected T value;
  protected LKind kind;

  public L(T value) {
    this.value = value;
    this.kind = LKind.INNER;
  }

  public L() {
    this.value = null;
    this.kind = LKind.BOT;
  }

  public L<T> bottom() {
    return new L();
  }

  public L<T> top() {
    L l = new L();
    l.kind = LKind.TOP;
    return l;
  }

  public L<T> inner(T value) {
    return new L(value);
  }

  public boolean isInner() {
    return kind == LKind.INNER;
  }

  public T unwrap() {
    switch (kind) {
      case BOT:
        throw new RuntimeException("Try to unwrap the bottom element of `L<T>`.");
      case TOP:
        throw new RuntimeException("Try to unwrap the top element of `L<T>`.");
      default: return value;
    }
  }

  public L<T> copy() {
    switch (kind) {
      case BOT: return bottom();
      case TOP: return top();
      default: return copy_inner("copy", value);
    }
  }

  private L<T> copy_inner(String from, Object toCopy) {
    Copy v = Cast.toCopy("The operation `L<T>." + from + "` requires the type `T` to implement `Copy`.", toCopy);
    return inner((T) v.copy());
  }

  public boolean equals(Object obj) {
    L<T> other = flatLatticeOf("equals", obj);
    return kind == other.kind && value.equals(other.value);
  }

  public Kleene entail(Object obj) {
    L<T> other = flatLatticeOf("entail", obj);
    switch (other.kind) {
      case BOT: return Kleene.TRUE;
      case TOP: return Kleene.fromBool(kind == LKind.TOP);
      default:
        return entail_inner(other.value);
    }
  }

  private Kleene entail_inner(T other) {
    switch (kind) {
      case BOT: return Kleene.FALSE;
      case TOP: return Kleene.TRUE;
      default: {
        assertSameInnerTypes("entail", value, other);
        if (value.equals(other)) {
          return Kleene.TRUE;
        }
        else {
          return Kleene.UNKNOWN;
        }
      }
    }
  }

  public void join_in_place(Object obj) {
    L<T> v = join(obj);
    this.value = v.value;
    this.kind = v.kind;
  }

  public L<T> join(Object obj) {
    L<T> other = flatLatticeOf("join", obj);
    switch (other.kind) {
      case BOT: return copy();
      case TOP: return top();
      default: return join_inner(other.value);
    }
  }

  public L<T> join_inner(T other) {
    checkNull("join", other);
    switch (this.kind) {
      case BOT: return copy_inner("join", other);
      case TOP: return top();
      default: {
        assertSameInnerTypes("join", value, other);
        if(value.equals(other)) {
          return copy_inner("join", other);
        }
        else {
          return top();
        }
      }
    }
  }

  public void meet_in_place(Object obj) {
    L<T> v = meet(obj);
    this.value = v.value;
    this.kind = v.kind;
  }

  public L<T> meet(Object obj) {
    L<T> other = flatLatticeOf("meet", obj);
    switch (other.kind) {
      case BOT: return bottom();
      case TOP: return copy();
      default: return meet_inner(other.value);
    }
  }

  public L<T> meet_inner(T other) {
    checkNull("meet", other);
    switch (this.kind) {
      case BOT: return bottom();
      case TOP: return copy_inner("meet", other);
      default: {
        assertSameInnerTypes("meet", value, other);
        if(value.equals(other)) {
          return copy_inner("meet", other);
        }
        else {
          return bottom();
        }
      }
    }
  }

  private void assertSameInnerTypes(String from, T self, Object other) {
    checkNull(from, other);
    if (!self.getClass().isInstance(other)) {
      throw new RuntimeException(
        "Undefined entailment relation between `" +
        this.getClass().getCanonicalName() +
        "` and `" +
        other.getClass().getCanonicalName() + "`");
    }
  }

  private L<T> flatLatticeOf(String from, Object obj) {
    checkNull(from, obj);
    if (obj instanceof L) {
      return (L) obj;
    }
    else {
      return inner((T) obj);
    }
  }

  private void checkNull(String from, Object obj) {
    if (obj == null) {
      throw new NullPointerException("Operation `L<T>." + from + "` does not accept a `null` argument.");
    }
  }

  public String toString() {
    switch (kind) {
      case BOT: return "bottom";
      case TOP: return "top";
      default: return value.toString();
    }
  }
}
