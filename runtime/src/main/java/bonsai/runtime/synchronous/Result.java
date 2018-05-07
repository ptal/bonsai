// Copyright 2017 Pierre Talbot (IRCAM)

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
import bonsai.runtime.synchronous.statements.*;

public class Result {
  private CompletionCode code;
  private ArrayList<SpaceStmt> branches;

  public Result(CompletionCode code, ArrayList<SpaceStmt> branches) {
    this.code = code;
    this.branches = branches;
  }

  public Result(CompletionCode code, SpaceStmt branch) {
    this(code);
    this.branches.add(branch);
  }

  public Result(CompletionCode code) {
    this(code, new ArrayList());
  }

  public CompletionCode code() {
    return code;
  }

  public ArrayList<SpaceStmt> branches() {
    return branches;
  }
}
