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

package bonsai.runtime.synchronous;

import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class SpaceMachine
{
  private Program body;
  private Environment env;

  public SpaceMachine(Program body) {
    this.body = body;
    this.env = new Environment();
  }

  public void prepare() {
    body.prepareInstantSub(env, 0);
  }

  // Returns `true` if the program is paused (through a `stop` or `pause up` statement).
  // If the program is terminated, it returns `false`.
  public boolean execute() {
    CompletionCode code = body.executeSub(env, 0);
    return code != CompletionCode.TERMINATE;
  }
}
