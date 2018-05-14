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

#[error(E0029, 30, 20)]
#[error(E0029, 31, 20)]
#[error(E0029, 32, 20)]
#[error(E0029, 33, 20)]
#[error(E0029, 34, 20)]
#[error(E0029, 35, 20)]
#[error(E0029, 39, 20)]

package test;

public class E0029 // NonInstantaneousSpace
{
  public single_space LMax a;
  public single_space LMax b;

  proc test_ko1() = space pause end
  proc test_ko2() = space pause up end
  proc test_ko3() = space stop end
  proc test_ko4() = space loop pause end end
  proc test_ko5() = space suspend when a |= b in nothing end end
  proc test_ko6() = space when a |= b then nothing else pause end end
  proc test1() = nothing
  proc test2() = pause
  proc test_ok1() = space run test1() end
  proc test_ko8() = space run test2() end
}
