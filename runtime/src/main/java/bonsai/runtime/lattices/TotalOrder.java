// Copyright 2018 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.runtime.lattices;

import bonsai.runtime.core.*;

// `T` must override `equals`.
public abstract class TotalOrder<T>
  implements Lattice, Copy<TotalOrder>, Restorable
{
  protected T value;

  public TotalOrder(T v) {
    this.value = v;
  }

  // Returns `true` if `this.value |= other.value`.
  protected abstract boolean entail_inner(TotalOrder<T> other);

  // Access: READ(this)
  public T unwrap() {
    return this.value;
  }

  public TotalOrder<T> join(Object o) {
    TotalOrder<T> v = castTotalOrder("join", o);
    return entail_inner(v) ? this.copy() : v.copy();
  }

  public void join_in_place(Object o) {
    this.value = join(o).value;
  }

  public TotalOrder<T> meet(Object o) {
    TotalOrder<T> v = castTotalOrder("meet", o);
    return entail_inner(v) ? v.copy() : this.copy();
  }

  public void meet_in_place(Object o) {
    this.value = meet(o).value;
  }

  public Object label() {
    return copy();
  }

  public void restore(Object label) {
    TotalOrder<T> v = castTotalOrder("restore", label);
    this.value = v.value;
  }

  public Kleene entail(Object o) {
    TotalOrder<T> v = castTotalOrder("entail", o);
    if (entail_inner(v)) {
      return Kleene.TRUE;
    }
    else {
      return Kleene.FALSE;
    }
  }

  public boolean equals(Object o) {
    TotalOrder<T> v = castTotalOrder("equals", o);
    return value.equals(v.value);
  }

  private TotalOrder<T> castTotalOrder(String from, Object o) {
    if (o instanceof TotalOrder) {
      return (TotalOrder<T>) o;
    }
    else {
      throw new ClassCastException("Operation `" + from + "` between type `TotalOrder` and type `"
        + o.getClass().getCanonicalName() + "` is not supported.");
    }
  }

  public String toString() {
    if (this.equals(this.bottom())) {
      return "bot";
    }
    else if (this.equals(this.top())) {
      return "top";
    }
    else {
      return value.toString();
    }
  }
}
