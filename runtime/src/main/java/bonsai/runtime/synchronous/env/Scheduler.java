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

public class Scheduler
{
  private HashMap<Event, ArrayList<Schedulable>> waitingQueue;

  public Scheduler() {
    waitingQueue = new HashMap();
  }

  public void subscribe(Event event, Schedulable program) {
    waitingQueue
      .computeIfAbsent(event, k -> new ArrayList<>())
      .add(program);
  }

  public void schedule(Event event) {
    ArrayList<Schedulable> programs = waitingQueue.get(event);
    for (Schedulable s: programs) {
      s.schedule(null);
    }
    programs.clear();
  }
}
