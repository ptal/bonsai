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

package bonsai.runtime.synchronous.env;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.variables.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.statements.SpaceStmt;

public class Layer
{
  private Space space;
  private Scheduler scheduler;

  public Layer()
  {
    space = new Space();
    scheduler = new Scheduler();
  }

  public Variable lookUpVar(String uid) {
    return space.lookUpVar(uid);
  }

  public void subscribe(Event event, Schedulable program) {
    scheduler.subscribe(event, program);
  }

  public void schedule(Event event) {
    scheduler.schedule(event);
  }

  // See Scheduler.processWasScheduled
  public boolean processWasScheduled() {
    return scheduler.processWasScheduled();
  }

  public boolean unblock(Program body) {
    return false;
  }

  public void enterScope(String uid, Object defaultValue, Consumer<Object> refUpdater) {
    space.enterScope(uid, defaultValue, refUpdater);
  }

  public void exitScope(String uid) {
    space.exitScope(uid);
  }

  public void register(String uid, boolean overwrite) {
    space.register(uid, overwrite);
  }
}
