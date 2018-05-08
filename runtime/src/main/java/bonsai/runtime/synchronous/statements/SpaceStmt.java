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

/// `Space` represents the spacetime statement `space b1 || ... || bn end`.
/// The fields `branches` is the code of the branches `{b1,...,bn}` and `singleTimeClosure` contains the variables annotated with `single_time` captured in any of these branches.

package bonsai.runtime.synchronous.statements;

import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.*;

public class SpaceStmt extends Instruction
{
  private ArrayList<String> singleTimeClosure;
  private Program branch;

  public SpaceStmt(ArrayList<String> singleTimeClosure, Program branch) {
    super();
    this.singleTimeClosure = singleTimeClosure;
    this.branch = branch;
  }

  public SpaceStmt copy() {
    return new SpaceStmt(
      (ArrayList<String>) singleTimeClosure.clone(),
      branch.copy());
  }

  public Result execute(SpaceEnvironment env) {
    return new Result(CompletionCode.TERMINATE, this);
  }

  public void countReadWrite(SpaceEnvironment env) {}

  public ArrayList<String> singleTimeClosure() {
    return singleTimeClosure;
  }

  public Program branch() {
    return branch;
  }
}
