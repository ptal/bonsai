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

public class Delay extends ASTNode implements Program
{
  private final CompletionCode kind;
  private CompletionCode k;
  private boolean nextInstant;
  public Delay(CompletionCode kind) {
    super();
    this.kind = kind;
    prepare();
  }

  public void prepare() {
    k = CompletionCode.WAIT;
    nextInstant = false;
  }

  public void canInstant(int layersRemaining, Layer layer) {
    checkNoSubLayer(layersRemaining, "Delay.canInstant");
    nextInstant = true;
  }

  public boolean canTerminate() {
    return k == CompletionCode.TERMINATE || (k == kind && nextInstant);
  }

  public void abort(Layer layer) {}
  public void suspend(Layer layer) {}

  public CompletionCode execute(int layersRemaining, Layer layer){
    checkNoSubLayer(layersRemaining, "Delay.execute");
    if (k == CompletionCode.WAIT) {
      k = kind;
    }
    else if (k == kind) {
      k = CompletionCode.TERMINATE;
    }
    return k;
  }

  public boolean canWriteOn(int layersRemaining, String uid, boolean inSurface) {
    checkNoSubLayer(layersRemaining, "Delay.canWriteOn");
    return false;
  }

  public int countLayers() { return 0; }
}
