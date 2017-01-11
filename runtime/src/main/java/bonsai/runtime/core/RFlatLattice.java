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

package bonsai.runtime.core;

import java.util.Optional;

public class RFlatLattice<T extends Restorable & Copy<T>> extends FlatLattice<T>
  implements Restorable, Copy<RFlatLattice<T>>
{
  public RFlatLattice(T value) {
    super(value);
  }

  public RFlatLattice() {
    super();
  }

  public Object label() {
    if (isBottom()) {
      return bottom();
    }
    else {
      return Optional.of(value.get().label());
    }
  }

  public void restore(Object label) {
    Optional l = (Optional) label;
    if (l.isPresent()){
      this.value.get().restore(l.get());
    }
    else {
      this.value = l;
    }
  }

  public RFlatLattice<T> copy() {
    if (isBottom()) {
      return bottom();
    }
    else {
      return new RFlatLattice(value.get().copy());
    }
  }

  public RFlatLattice<T> bottom() {
    return new RFlatLattice<T>();
  }
}