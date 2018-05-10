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

#[error(E0016, 33, 4)]
#[error(E0016, 34, 4)]
#[error(E0016, 36, 15)]
#[error(E0016, 36, 22)]

package test;

public class ForbiddenWriteOnPre
{
  ref single_space T a;
  single_space T ok;

  public ForbiddenWriteOnPre(T a) {
    this.a = a;
  }

  proc test() =
    world_line N b;
    pre a <- 1;
    pre b <- 2;
    ok <- pre a;
    J.external(pre b, pre a, ok);
  end
}
