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

public class LMax extends LatticeVar implements Restorable, Copy<LMax>, Resettable<LMax>
{
  private Integer value;

  public LMax() {
    this.value = new Integer(0);
  }

  private LMax(LMax l) {
    this.value = l.value;
  }

  public Object label() {
    return copy();
  }

  public void restore(Object label) {
    this.value = (Integer) label;
  }

  public LMax copy() {
    return new LMax(this);
  }

  public void reset(LMax i) {
    this.value = i.value;
  }

  public void inc() {
    this.value += 1;
  }

  public Integer get() {
    return this.value;
  }

  public void join(Object o) {
    LMax v = castLMax("join", o);
    this.value = (this.value > v.value) ? this.value : v.value;
  }

  public EntailmentResult entail(Object o) {
    LMax v = castLMax("entail", o);
    if (value >= v.value) {
      return EntailmentResult.TRUE;
    }
    else {
      return EntailmentResult.UNKNOWN;
    }
  }

  public LatticeVar bottom() {
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