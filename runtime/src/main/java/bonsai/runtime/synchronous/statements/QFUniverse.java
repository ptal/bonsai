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

package bonsai.runtime.synchronous;

import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class QFUniverse extends ASTNode
{
  private Program body;

  public QFUniverse(Program body) {
    super();
    this.body = body;
  }

  public void prepareSubInstant(Environment env, int layerIndex) {
    env.traverseLayerPrepare(layerIndex, body::prepareInstant, body::prepareSubInstant);
  }

  public CompletionCode executeSub(Environment env, int layerIndex) {
    return env.traverseLayer(layerIndex, body::execute, body::executeSub);
  }

  public void prepareInstant(Layer layer) {}

  public CompletionCode execute(Layer layer) {
    return CompletionCode.PAUSE_DOWN;
  }

  public void meetRWCounter(Layer layer) {}

  public CanResult canWriteOn(String uid, boolean inSurface) {
    return null;
  }
}
