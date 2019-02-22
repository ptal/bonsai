// Copyright 2019 Pierre Talbot

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

import bonsai.runtime.synchronous.env.Layer;

public interface BModule
{
  /// Initialize the UIDs of field variables.
  public void __init();
  /// Wrap the process `proc` into variable declarations (such as `SingleSpaceVarDecl`) representing the fields of the class.
  /// If the current module is root (directly passed to `SpaceMachine`), then the reference fields are treated as normal fields.
  public Statement __wrap_process(boolean isRoot, Statement proc);
}
