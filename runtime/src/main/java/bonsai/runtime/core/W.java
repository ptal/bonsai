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

public class W<T extends Restorable> extends L<T>
  implements Restorable
{
  public W(T value) {
    super(value);
  }

  public W() {
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

  /// Shared label property: either the label equal bottom and is created from scratch (`bottom()`) or it depends on the underlying value (inductively).
  public void restore(Object label) {
    Optional l = (Optional) label;
    if (l.isPresent()){
      this.value.get().restore(l.get());
    }
    else {
      this.value = Optional.empty();
    }
  }

  public L<T> copy() {
    if (isBottom()) {
      return bottom();
    }
    else {
      Copy v = Cast.toCopy("<anon> in W.copy", value.get());
      return new W((T) v.copy());
    }
  }

  public W<T> bottom() {
    return new W<T>();
  }
}