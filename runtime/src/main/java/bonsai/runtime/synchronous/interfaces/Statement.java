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

import java.util.*;
import bonsai.runtime.core.Copy;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;

/// Global conditions:
///   - if layersRemaining == 0: Must never enter a sublayer.
///   - if layersRemaining > 0:  Must call substatements where no sublayer is currently active.
///
public interface Statement extends Schedulable, Copy<Statement>
{
  /// This method is called to (re)initialize the statement.
  /// It is called at start (before any other method) and is called again if the object was used and need to be reset.
  /// This call must be forwarded to all sub-components: it ignores layers and instants.
  /// Special:
  ///   - Instantaneous statements should have nothing to prepare since they are stateless.
  void prepare();

  /// Called at the beginning of an instant of a layer if this statement *can* be reached.
  /// Purposes:
  ///   - Count all the possible read/write operations (an upper bound) and add them in `layer`.
  ///   - Register variables in `Layer.Space`.
  /// Conditions:
  ///   - It is not idempotent on `layer`.
  void canInstant(int layersRemaining, Layer layer);

  /// Collect all the queues currently active.
  /// This method is called at the beginning of an instant before `canInstant` (although it should not matter).
  /// `SpaceMachine` will pop the queues returned.
  HashSet<String> activeQueues(int layersRemaining);

  boolean canTerminate();

  /// Called on a statement to suspend it.
  /// Purpose:
  ///   - Reduce the read/write counters.
  /// Conditions:
  ///   - Must never enter a sublayer.
  ///   - It is not idempotent.
  void suspend(Layer layer);

  /// Called on a statement to abort it.
  /// We have in addition to the purpose and conditions of `suspend(Layer)`:
  /// Purpose:
  ///   - Exit the scope of the variables.
  void abort(Layer layer);

  /// Called several time during an instant if the statement *must* be executed.
  /// It must always return the completion code of the layer pointed by `layersRemaining`.
  /// Conditions:
  ///   - It is not idempotent: it can be viewed as an internal transition of the statement from the current state to the next state.
  StmtResult execute(int layersRemaining, Layer layer);

  /// We analyse the current program to prove that a write on `uid` still can happen.
  /// We suppose the conditions currently suspended on `uid` to be `false` (if `inSurface` is `true`).
  /// `inSurface` is `true` if the flow of control is on the statement currently analysed.
  /// It returns `true` if a write can happen, `false` otherwise.
  /// Note:
  ///   - It can return `true` immediately when one of its sub-component returns `true`.
  /// Conditions:
  ///   - Must be idempotent
  ///   - The internal state of the statement must not be modified.
  boolean canWriteOn(int layersRemaining, String uid, boolean inSurface);

  /// Returns the number of layers of the program.
  /// This method is only called once before `prepare`.
  int countLayers();
}
