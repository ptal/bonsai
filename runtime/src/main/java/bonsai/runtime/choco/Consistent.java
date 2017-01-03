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

package bonsai.runtime.choco;

//  Lattice represented by this class is:
//        Top
//     /   |   \
//    /    |    \
// True Unknown False
//     \   |    /
//      \  |   /
//       Bottom

public enum Consistent {
  Unknown,
  True,
  False;

  public String toString() {
    switch(this) {
      case Unknown: return "unknown";
      case True: return "true";
      default: return "false";
    }
  }
}
