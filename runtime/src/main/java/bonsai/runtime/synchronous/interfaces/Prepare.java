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

import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;

public interface Prepare
{
  // This method is called before the statement is executed.
  // `prepare` is not called again until termination (terminated completion code for statements (or aborted), and returning a non-suspended value for expressions).
  void prepare(Layer env);
}
