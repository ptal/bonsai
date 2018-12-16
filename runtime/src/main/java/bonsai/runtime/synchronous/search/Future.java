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

package bonsai.runtime.synchronous.search;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.statements.*;
import bonsai.runtime.synchronous.interfaces.*;

public class Future {
  public Statement body;
  public CapturedSpace space;

  public Future(Statement body, CapturedSpace space) {
    this.body = body;
    this.space = space;
  }

  public static Future merge(List<Future> futures) {
    ArrayList<Statement> processes = new ArrayList(futures.size());
    CapturedSpace merged_space = new CapturedSpace();
    for(Future future : futures) {
      processes.add(future.body);
      merged_space.merge(future.space);
    }
    Statement merged_proc;
    if (processes.size() > 1) {
      merged_proc = new DisjunctivePar(processes, 0);
    }
    else {
      merged_proc = processes.get(0);
    }
    return new Future(merged_proc, merged_space);
  }
}
