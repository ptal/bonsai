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

public class LMax extends TotalOrder<Integer>
{
  static Integer BOT = Integer.MIN_VALUE;
  static Integer TOP = Integer.MAX_VALUE;

  public LMax() {
    super(BOT);
  }

  public LMax(Integer v) {
    super(v);
  }

  public LMax(LMax m) {
    super(new Integer(m.value));
  }

  public LMax bottom() {
    return new LMax(BOT);
  }

  public LMax top() {
    return new LMax(TOP);
  }

  public boolean isBottom() {
    return value == BOT;
  }

  public boolean isTop() {
    return value == TOP;
  }

  // Access: READWRITE(this)
  public void inc() {
    if (!this.equals(top())) {
      this.value += 1;
    }
  }

  public LMax copy() {
    return new LMax(this);
  }

  public LMax join(Object o) {
    return (LMax) super.join(wrapInteger(o));
  }

  public LMax meet(Object o) {
    return (LMax) super.meet(wrapInteger(o));
  }

  public Kleene entails(Object o) {
    return super.entails(wrapInteger(o));
  }

  protected boolean entails_inner(TotalOrder<Integer> o) {
    Integer v = castInteger("entails_inner", o.value);
    return value >= v;
  }

  private Integer castInteger(String from, Object o) {
    if (o instanceof Integer) {
      return (Integer) o;
    }
    else {
      throw new ClassCastException("Operation `" + from + "` between type `Integer` (in `LMax`) and type `"
        + o.getClass().getCanonicalName() + "` is not supported.");
    }
  }
}
