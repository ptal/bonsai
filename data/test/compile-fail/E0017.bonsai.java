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

#[error(E0017, 35, 10)]
#[error(E0017, 36, 10)]
#[error(E0017, 37, 10)]
#[error(E0017, 38, 9)]
#[error(E0017, 38, 18)]

package test;

public class PreOnlyOnStream
{
  JavaHost h = new JavaHost();
  module Module m;
  single_space T ok;

  public PreOnlyOnStream(T a) {
    this.a = a;
  }

  proc test() =
    single_time N b;
    ok <- pre b;
    ok <- pre m;
    ok <- pre h;
    when pre b |= pre m then nothing end
  end
}
