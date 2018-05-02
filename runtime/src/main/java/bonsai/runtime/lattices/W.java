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

package bonsai.runtime.lattices;

import bonsai.runtime.core.*;

public class W<T extends Restorable> extends L<T>
  implements Restorable
{
  public W(T value) {
    super(value);
  }

  public W() {
    super();
  }

  public W<T> bottom() {
    return new W();
  }

  public W<T> top() {
    W w = new W();
    w.kind = LKind.TOP;
    return w;
  }

  public W<T> inner(T value) {
    return new W(value);
  }

  // We encapsulate the label produced by the inner element inside a flat lattice.
  // This is required to save the bottom and top elements.
  public Object label() {
    switch (kind) {
      case BOT: return super.bottom();
      case TOP: return super.top();
      default:
        return new L(value.label());
    }
  }

  /// Shared label property: either the label equals bottom or top and is created from scratch (`bottom()`) or it depends on the underlying value (inductively).
  public void restore(Object label) {
    L l = (L) label;
    switch (l.kind) {
      case BOT: meet_in_place(bottom());
      case TOP: join_in_place(top());
      default: {
        this.value.restore(l.value);
        this.kind = LKind.INNER;
      }
    }
  }
}