// Copyright 2017 Pierre Talbot (IRCAM)

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

import bonsai.runtime.lattice.*;

public class LMax implements Lattice, Restorable, Copy<LMax>
{
  private Integer value;

  public LMax() {
    this.value = new Integer(0);
  }

  public LMax(Integer v) {
    this.value = v;
  }

  private LMax(LMax l) {
    this.value = l.value;
  }

  // Access: READWRITE(this)
  public void inc() {
    this.value += 1;
  }

  // Access: READ(this)
  public Integer get() {
    return this.value;
  }

  public LMax meet(Object o) {
    LMax v = castLMax("meet", o);
    return new LMax((this.value < v.value) ? this : v);
  }

  public void meet_in_place(Object o) {
    this.value = meet(o).value;
  }

  public void join_in_place(Object o) {
    this.value = join(o).value;
  }

  public LMax join(Object o) {
    LMax v = castLMax("join", o);
    return new LMax((this.value > v.value) ? this : v);
  }

  public Object label() {
    return copy();
  }

  /// Shared label property: `Integer` label are automatically copied.
  public void restore(Object label) {
    this.value = (Integer) label;
  }

  public LMax copy() {
    return new LMax(this);
  }

  public Kleene entail(Object o) {
    LMax v = castLMax("entail", o);
    if (value >= v.value) {
      return Kleene.TRUE;
    }
    else {
      return Kleene.UNKNOWN;
    }
  }

  public Lattice bottom() {
    return new LMax();
  }

  private LMax castLMax(String from, Object o) {
    if (o instanceof LMax) {
      return (LMax) o;
    }
    else { throw new RuntimeException(
      "Unsupported " + from + " operation on LMax"); }
  }

  public String toString() {
    return value.toString();
  }
}