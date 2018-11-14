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

package bonsai.runtime.synchronous.statements;

import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public class Pause extends ASTNode implements Program
{
  private CompletionCode k;
  public Pause() {
    super();
    prepare();
  }

  public void prepare() {
    k = CompletionCode.WAIT;
  }

  public void canInstant(int layersRemaining, Layer layer) {
    checkNoSubLayer(layersRemaining, "Pause.canInstant");
  }

  public boolean canTerminate() {
    return k == CompletionCode.TERMINATE;
  }

  public void abort(Layer layer) {}
  public void suspend(Layer layer) {}

  public CompletionCode execute(int layersRemaining, Layer layer){
    checkNoSubLayer(layersRemaining, "Pause.execute");
    if (k == CompletionCode.WAIT) {
      k = CompletionCode.PAUSE;
    }
    else if (k == CompletionCode.PAUSE) {
      k = CompletionCode.TERMINATE;
    }
    return k;
  }

  public boolean canWriteOn(int layersRemaining, String uid, boolean inSurface) {
    checkNoSubLayer(layersRemaining, "Pause.canWriteOn");
    return false;
  }

  public int countLayers() { return 0; }
}
