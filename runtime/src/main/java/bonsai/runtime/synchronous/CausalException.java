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

import java.util.*;

public class CausalException extends RuntimeException {
  public CausalException(String source) {
    super("[BUG]: The SpaceMachine was unable to schedule some processes during execution.\n" +
          "It implies that the program is not causal (bug in the static analysis of the compiler or in the implementation of the scheduling).\n"+
          "Source: " + source);
  }
}
