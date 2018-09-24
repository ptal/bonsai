// Copyright 2018 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.runtime.synchronous;

import bonsai.runtime.synchronous.variables.Variable;

enum EventKind {
  CAN_READ,
  CAN_READWRITE
}

public class Event {
  private EventKind kind;
  private Integer uid;

  public Event(EventKind kind, Integer uid) {
    this.kind = kind;
    this.uid = uid;
  }

  public static Event makeCanRead(Variable var) {
    return new Event(EventKind.CAN_READ, var.uid());
  }

  public static Event makeCanReadWrite(Variable var) {
    return new Event(EventKind.CAN_READWRITE, var.uid());
  }

  public int hashCode() {
    return uid + ((kind == EventKind.CAN_READ) ? 0: 1000000);
  }

  public boolean equals(Object obj) {
    if (obj == null) {
      return false;
    }
    else if (getClass() != obj.getClass()) {
      return false;
    }
    else {
      Event e = (Event) obj;
      return kind == e.kind && uid == e.uid;
    }
  }
}
