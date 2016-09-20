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

package bonsai.chococubes.choco;

public class ConsistentLattice {
  private Consistent value;

  public ConsistentLattice(Consistent value) {
    this.value = value;
  }

  public static ConsistentLattice bottom() {
    return new ConsistentLattice(Consistent.Bottom);
  }

  public EntailmentResult entails(ConsistentLattice other) {
    return this.entails(other.value);
  }

  public EntailmentResult entails(Consistent other) {
    if (other == Consistent.Bottom || this.value == other) {
      return EntailmentResult.True;
    }
    else if (this.value == Consistent.Bottom) {
      return EntailmentResult.Unknown;
    }
    else {
      return EntailmentResult.False;
    }
  }

  public void tell(ConsistentLattice other) {
    this.tell(other.value);
  }

  public void tell(Consistent other) {
    assert this.value == other || this.value == Consistent.Bottom || other == Consistent.Bottom :
      "Reached TOP element in consistent lattice.";
    if (other != Consistent.Bottom) {
      this.value = other;
    }
  }
}
