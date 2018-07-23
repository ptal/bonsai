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

#[error(E0030, 23, 14)]
#[error(E0030, 25, 14)]
#[error(E0030, 29, 14)]

package test;

public class E0030
{
  public proc test_ko1() = run test_ko1()

  public proc test_ko2() = run test_ko2_1()

  public proc test_ko2_1() = run test_ko2()

  public proc test_ko3() =
    when true then
      module E0030 m = new E0030();
      run m.test_ko3();
    end

  // The error is caugth in test_ko1, we do not repeat it.
  public proc test_ok4() =
    module E0030 m = new E0030();
    run m.test_ko1();
  end
}
