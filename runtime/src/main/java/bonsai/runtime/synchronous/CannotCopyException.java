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

public class CannotCopyException extends RuntimeException {
  public CannotCopyException(String from) {
    super("[BUG] Copy is only useful to copy program `p` appearing in `space p end`.\n" +
          "The statement `" + from + "` cannot appear in `p`.\n" +
          "This should be checked by the compiler.");
  }
}