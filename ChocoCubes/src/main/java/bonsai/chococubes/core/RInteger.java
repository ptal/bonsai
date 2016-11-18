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

package bonsai.chococubes.core;

public class RInteger implements Restorable
{
  private Integer value;

  public RInteger(int value) {
    this.value = new Integer(value);
  }

  public Object label() {
    return new Integer(this.value);
  }

  public void restore(Object label) {
    this.value = (Integer) label;
  }

  public String toString() {
    return value.toString();
  }
}