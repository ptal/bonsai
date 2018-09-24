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

package bonsai.runtime.synchronous.variables;

public class RWCounter {
  public int write;
  public int readwrite;
  public int read;

  public RWCounter(int write, int readwrite, int read) {
    this.write = write;
    this.readwrite = readwrite;
    this.read = read;
  }

  public boolean isReadable() {
    return this.write == 0 && this.readwrite == 0;
  }

  public boolean isReadWritable() {
    return this.write == 0;
  }
}
