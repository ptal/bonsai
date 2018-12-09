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

package bonsai.runtime.synchronous.env;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.variables.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.expressions.Entailment;

public class Scheduler
{
  private HashMap<Event, ArrayList<Schedulable>> waitingList;
  private boolean scheduledProcess;
  private HashMap<String, PartialCond> unblocked;

  class PartialCond {
    public Entailment cond;
    public Kleene result;
    public PartialCond(Entailment cond, Kleene result) {
      this.cond = cond;
      this.result = result;
    }
  }

  public Scheduler() {
    waitingList = new HashMap();
    scheduledProcess = false;
    unblocked = new HashMap();
  }

  public void subscribe(Event event, Schedulable process) {
    waitingList
      .computeIfAbsent(event, k -> new ArrayList<>())
      .add(process);
  }

  // When `schedule` is called, the processes registered on this event are removed from the `waitingList`;
  public void schedule(Event event) {
    ArrayList<Schedulable> processes = waitingList.get(event);
    if (processes != null) {
      for (Schedulable s: processes) {
        s.schedule(null);
        scheduledProcess = true;
      }
      processes.clear();
    }
  }

  // Return true if a process has been scheduled since the last call to this method.
  public boolean processWasScheduled() {
    if (scheduledProcess) {
      scheduledProcess = false;
      return true;
    } else {
      return false;
    }
  }

  public void subscribeUnblocked(String uid, Entailment entailment, Kleene result) {
    unblocked.put(uid, new PartialCond(entailment, result));
  }

  public void scheduleUnblocked(String uid) {
    PartialCond partial = unblocked.remove(uid);
    if (partial != null) {
      partial.cond.commit(partial.result);
    }
  }

  public void unsubscribeUnblocked(String uid) {
    unblocked.remove(uid);
  }
}
