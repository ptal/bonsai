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

package bonsai.runtime.synchronous;

public enum CompletionCode {
  PAUSE_DOWN,
  WAIT,
  STOP,
  PAUSE_UP,
  PAUSE,
  TERMINATE;

  public String toString() {
    switch(this) {
      case PAUSE_DOWN: return "program waiting to execute a sublayer (this is an internal state)";
      case WAIT: return "program waiting an event inside an instant (this is an internal state)";
      case PAUSE: return "pause";
      case PAUSE_UP: return "pause up";
      case STOP: return "stop";
      default: return "program termination";
    }
  }

  public boolean isInternal() {
    switch(this) {
      case WAIT:
      case PAUSE_DOWN: return true;
      default: return false;
    }
  }

  public boolean isLayerTerminated() {
    switch(this) {
      case PAUSE_UP:
      case STOP:
      case TERMINATE: return true;
      default: return false;
    }
  }

  public CompletionCode merge(CompletionCode k) {
    return CompletionCode.values()[Math.min(k.ordinal(), this.ordinal())];
  }
}
