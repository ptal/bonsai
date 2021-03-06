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

package bonsai.runtime.synchronous.interfaces;

import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;

public interface Schedulable
{
  void setParent(Schedulable parent);
  // `schedule` can be called even if the current process is terminated.
  // see Scheduler.schedule.
  // It notifies the current process that an event arrived on a variable of one of its sub-expressions.
  // Therefore it can be reschuled for execution.
  // Note: this method should only be of interest in the parallel statements (to avoid rescheduling processes if nothing changed).
  void schedule(Schedulable from);
}
