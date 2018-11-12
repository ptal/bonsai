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

public interface CanAnalysis
{
  // We analyse the current program to prove that a write on `uid` still can happen.
  // We suppose the conditions currently suspended on `uid` to be `false` (if `inSurface` is `true`).
  // `inSurface` is `true` if the statement analysed is currently suspended.
  // This method can only be called on non-terminated statements or suspended expressions.
  CanResult canWriteOn(String uid, boolean inSurface);
  // `true` if it can terminate, count all the possible read/write operations (an upper bound) and add them in `layer`.
  boolean canAnalysis(Layer layer);
  // Reduce the read/write counter of the terminated (or aborted) operations.
  boolean terminate(Layer layer);
}
