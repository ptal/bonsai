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

import bonsai.runtime.synchronous.interfaces.*;

public abstract class ASTNode implements Schedulable
{
  protected Schedulable parent;

  public ASTNode() {
    parent = null;
  }

  public void setParent(Schedulable parent) {
    this.parent = parent;
  }

  public void schedule(Schedulable from) {
    parent.schedule(from);
  }
}
