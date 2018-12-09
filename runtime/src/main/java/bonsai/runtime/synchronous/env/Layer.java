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
import bonsai.runtime.synchronous.expressions.Entailment;

public class Layer
{
  private Layer parent;
  private Space space;
  private Scheduler scheduler;
  private Optional<String> currentQueue;

  public Layer()
  {
    space = new Space();
    scheduler = new Scheduler();
    currentQueue = Optional.empty();
  }

  public Layer(Space space) {
    this();
    this.space = space;
  }

  public void setParent(Layer layer) {
    this.parent = layer;
  }

  public Layer parent() {
    return parent;
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

  public boolean unblock(Statement body, int layersRemaining) {
    return space.unblock(body, layersRemaining, this);
  }

  public void subscribeUnblocked(String uid, Entailment cond, Kleene result) {
    scheduler.subscribeUnblocked(uid, cond, result);
  }

  public void scheduleUnblocked(String uid) {
    scheduler.scheduleUnblocked(uid);
  }

  public void unsubscribeUnblocked(String uid) {
    scheduler.unsubscribeUnblocked(uid);
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

  public void enterQueue(String uid) {
    if(currentQueue.isPresent()) {
      throw new RuntimeException("[BUG] There is only one queue active at any time in a layer.");
    }
    currentQueue = Optional.of(uid);
  }

  public void exitQueue() {
    currentQueue = Optional.empty();
  }

  public String currentQueue() {
    if(!currentQueue.isPresent()) {
      throw new RuntimeException("[BUG] `Layer.currentQueue` can only be called when a queue is in scope.");
    }
    return currentQueue.get();
  }

  public Queueing getQueue(String name) {
    Variable queueVar = lookUpVar(name);
    return Cast.toQueueing(name, queueVar.value());
  }

  public HashMap<String, Variable> project(ArrayList<String> varsUIDs) {
    return space.project(varsUIDs);
  }
}
