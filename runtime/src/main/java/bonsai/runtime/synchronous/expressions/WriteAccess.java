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

package bonsai.runtime.synchronous.expressions;

import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.variables.*;

public class WriteAccess extends Access
{
  public WriteAccess(String uid) {
    super(uid);
    // System.out.println("write " + uid);
  }

  public WriteAccess copy() {
    return new WriteAccess(uid);
  }

  public void canInstant(Layer layer) {
    Variable var = layer.lookUpVar(uid);
    var.joinWrite(layer);
  }

  public void terminate(Layer layer) {
    Variable var = layer.lookUpVar(uid);
    var.meetWrite(layer);
  }

  // A write access is always possible.
  public ExprResult execute(Layer layer) {
    Variable var = layer.lookUpVar(uid);
    return new ExprResult(var.value());
  }

  public boolean canWriteOn(String uid) {
    return uid == this.uid;
  }
}
