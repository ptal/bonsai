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

package bonsai.runtime.lattices;

import bonsai.runtime.core.*;

public class LMin extends TotalOrder<Integer>
{
  static Integer BOT = Integer.MAX_VALUE;
  static Integer TOP = Integer.MIN_VALUE;

  public LMin() {
    super(BOT);
  }

  public LMin(Integer v) {
    super(v);
  }

  private LMin(LMin m) {
    super(new Integer(m.value));
  }

  public LMin bottom() {
    return new LMin(BOT);
  }

  public LMin top() {
    return new LMin(TOP);
  }

  public boolean isBottom() {
    return value == BOT;
  }

  public boolean isTop() {
    return value == TOP;
  }

  // Access: READWRITE(this)
  public void dec() {
    if (!this.equals(top())) {
      this.value -= 1;
    }
  }

  public LMin copy() {
    return new LMin(this);
  }

  public LMin join(Object o) {
    return (LMin) super.join(wrapInteger(o));
  }

  public LMin meet(Object o) {
    return (LMin) super.meet(wrapInteger(o));
  }

  public Kleene entails(Object o) {
    return super.entails(wrapInteger(o));
  }

  protected boolean entails_inner(TotalOrder<Integer> o) {
    Integer v = castInteger("entails_inner", o.value);
    return value <= v;
  }

  private Integer castInteger(String from, Object o) {
    if (o instanceof Integer) {
      return (Integer) o;
    }
    else {
      throw new ClassCastException("Operation `" + from + "` between type `Integer` (in `LMin`) and type `"
        + o.getClass().getCanonicalName() + "` is not supported.");
    }
  }
}
