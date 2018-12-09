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

package bonsai.runtime.synchronous.interfaces;

import bonsai.runtime.core.Copy;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;

/// Expressions are stateless across instants but can maintain a state during an instant.
public interface Expression extends Schedulable, Copy<Expression>
{
  /// Called at the beginning of an instant if this expression *can* be reached.
  /// /!\ It is not idempotent.
  /// Purposes:
  ///  - Count all the possible read/write operations (an upper bound) and add them in `layer`.
  void canInstant(Layer layer);

  /// This function is always called to indicate that the expression is terminated.
  /// /!\ It is not idempotent.
  /// Purpose:
  ///   - Reduce the read/write counters.
  /// When:
  ///   - This function is called when the statement is satisfied with the answer given by `execute`.
  ///   - When the statement must be aborted or suspended.
  void terminate(Layer layer);

  /// Called several time during an instant if the expression *must* be executed.
  /// Conditions:
  ///   - Must be idempotent once `!result.isSuspended()`.
  ExprResult execute(Layer layer);

  /// We analyse the current program to prove that a write on `uid` still can happen.
  /// It returns `true` if a write can happen, `false` otherwise.
  /// Conditions:
  ///   - Must be idempotent
  ///   - Must not modify the internal state of the process.
  boolean canWriteOn(String uid);
}
