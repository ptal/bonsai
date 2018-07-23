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

#[error(E0029, 30, 27)]
#[error(E0029, 31, 27)]
#[error(E0029, 32, 27)]
#[error(E0029, 33, 27)]
#[error(E0029, 34, 27)]
#[error(E0029, 35, 27)]
#[error(E0029, 39, 27)]

package test;

public class E0029 // NonInstantaneousSpace
{
  public single_space LMax a;
  public single_space LMax b;

  public proc test_ko1() = space pause end
  public proc test_ko2() = space pause up end
  public proc test_ko3() = space stop end
  public proc test_ko4() = space loop pause end end
  public proc test_ko5() = space suspend when a |= b in nothing end end
  public proc test_ko6() = space when a |= b then nothing else pause end end
  public proc test1() = nothing
  public proc test2() = pause
  public proc test_ok1() = space run test1() end
  public proc test_ko8() = space run test2() end
}
