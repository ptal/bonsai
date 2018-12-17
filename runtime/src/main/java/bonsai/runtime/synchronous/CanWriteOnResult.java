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

package bonsai.runtime.synchronous;

import java.util.*;

public class CanWriteOnResult {
  public boolean canTerminate;
  public boolean canWrite;

  public CanWriteOnResult(boolean canTerminate, boolean canWrite) {
    this.canTerminate = canTerminate;
    this.canWrite = canWrite;
  }

  public CanWriteOnResult join(CanWriteOnResult r) {
    this.canTerminate = this.canTerminate && r.canTerminate;
    this.canWrite = this.canWrite || r.canWrite;
    return this;
  }
}
