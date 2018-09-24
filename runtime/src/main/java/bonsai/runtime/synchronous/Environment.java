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
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.variables.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.statements.SpaceStmt;

public class Environment<Queue extends Queueing<Future>>
{
  private Space space;
  private Queue queue;
  private Scheduler scheduler;

  public Environment(Queue queue)
  {
    this.queue = queue;
    space = new Space();
    scheduler = new Scheduler();
  }

  /// We create and push the branches created in the current instant onto the queue.
  /// It corresponds to `push: Store(L3) -> L4` in the lattice hierarchy.
  /// Caution: this function must always be called even with an empty set of branches.
  /// Rational: this function marks the end of an instant, and therefore internal data might be reinitialized.
  /// @see Future
  public void pushBranches(/* ArrayList<SpaceStmt> branches */) {
    // ArrayList<Future> futures = Future.createFutures(branches, varsST, varsWL);
    // queue.push(futures);
  }

  public Optional<Future> pop() {
    // if(!queue.isEmpty()) {
    //   Future future = queue.pop();
    //   future.restoreWL(varsWL);
    //   return Optional.of(future);
    // }
    // else {
      return Optional.empty();
    // }
  }

  public boolean isEmpty() {
    return queue.isEmpty();
  }

  public int queueSize() {
    return queue.size();
  }

  // private HashMap<String, Variable> vars() {
  //   return vars;
  // }

  public Variable lookUpVar(String name) {
    return space.lookUpVar(name);
  }

  public void subscribe(Event event, Schedulable program) {
    scheduler.subscribe(event, program);
  }

  public void schedule(Event event) {
    scheduler.schedule(event);
  }
}
