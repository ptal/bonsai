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
  MICRO_STEP,
  STUCK,
  PAUSE,
  PAUSE_UP,
  STOP,
  TERMINATE;

  public String toString() {
    switch(this) {
      case MICRO_STEP: return "program running inside an instant (this is an internal state)";
      case STUCK: return "program stucks inside an instant (this is an internal state)";
      case PAUSE: return "pause";
      case PAUSE_UP: return "pause up";
      case STOP: return "stop";
      default: return "program termination";
    }
  }
}
