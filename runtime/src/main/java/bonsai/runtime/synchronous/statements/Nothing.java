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

package bonsai.runtime.synchronous.statements;

import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class Nothing extends ASTNode implements Program
{
  public Nothing() {
    super();
  }

  public void prepareSub(Environment env, int layerIndex) {
    throw new NoSubLayerException("Nothing.prepareSub");
  }
  public CompletionCode executeSub(Environment env, int layerIndex) {
    throw new NoSubLayerException("Nothing.executeSub");
  }

  public void prepare(Layer layer) {}

  public CompletionCode execute(Layer layer) {
    return CompletionCode.TERMINATE;
  }

  public CanResult canWriteOn(String uid, boolean inSurface) {
    return new CanResult(true, false);
  }

  public boolean canAnalysis(Layer layer) {
    return true;
  }

  public boolean terminate(Layer layer) {
    return true;
  }
}
