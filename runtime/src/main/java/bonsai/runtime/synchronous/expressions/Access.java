// Copyright 2018 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.runtime.synchronous.expressions;

import java.util.*;
import bonsai.runtime.core.Copy;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;

public abstract class Access extends ASTNode implements Expression
{
  protected final String uid;
  private boolean hasSubscribed;

  public Access(String uid) {
    this.uid = uid;
    hasSubscribed = false;
  }

  public void canInstant(Layer layer) {
    hasSubscribed = false;
  }

  protected void subscribe(Layer layer, int eventKind) {
    if (!hasSubscribed) {
      hasSubscribed = true;
      Event event = new Event(uid, eventKind);
      layer.subscribe(event, this);
    }
  }
}
