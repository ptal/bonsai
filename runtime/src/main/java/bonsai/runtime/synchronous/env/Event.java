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

package bonsai.runtime.synchronous.env;

import bonsai.runtime.synchronous.variables.*;

// The event `kind` is waited (or generated) on the variable with UID `uid`.
public class Event {
  private String uid;
  private int kind;

  public static int ANY = 0;
  public static int CAN_READ = 1;
  public static int CAN_READWRITE = 2;

  public Event(String uid, int kind) {
    if (kind != ANY && kind != CAN_READ && kind != CAN_READWRITE) {
      throw new RuntimeException("Event constructor: unknown event `kind` (value: " + kind + ")");
    }
    this.kind = kind;
    this.uid = uid;
  }

  public int hashCode() {
    return uid.hashCode() + 100000*kind;
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
      return kind == e.kind && uid.equals(e.uid);
    }
  }

  public String toString() {
    String kindS = "invalid";
    if (kind == ANY) { kindS = "ANY"; }
    else if (kind == CAN_READ) { kindS = "CAN_READ"; }
    else if (kind == CAN_READWRITE) { kindS = "CAN_READWRITE"; }
    return kindS + "(" + uid + ")";
  }
}
