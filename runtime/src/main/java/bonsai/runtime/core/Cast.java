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

import bonsai.runtime.lattice.Lattice;

public class Cast
{
  static private void checkLatticeInterface(String var, Object o) {
    if (!(o instanceof Lattice)) {
      unimplementedInterface(var, o, "Lattice");
    }
  }

  static private void checkRestorableInterface(String var, Object o) {
    if(!(o instanceof Restorable)) {
      unimplementedInterface(var, o, "Restorable");
    }
  }

  static private void checkCopyInterface(String var, Object o) {
    if (!(o instanceof Copy)) {
      unimplementedInterface(var, o, "Copy");
    }
  }

  static private void unimplementedInterface(String var, Object o, String interfaceName) {
    throw new RuntimeException(
      "The variable `" + var + "` does not implement the interface `"
      + interfaceName + "` which is required. Object: " + o);
  }

  static public Lattice toLattice(String var, Object o) {
    checkLatticeInterface(var, o);
    return (Lattice) o;
  }

  static public Restorable toRestorable(String var, Object o) {
    checkRestorableInterface(var, o);
    return (Restorable) o;
  }

  static public Copy toCopy(String var, Object o) {
    checkCopyInterface(var, o);
    return (Copy) o;
  }
}
