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

#[error(E0031, 32, 27)]
#[error(E0031, 33, 27)]
#[error(E0031, 40, 27)]
#[error(E0031, 43, 4)]
#[error(E0031, 46, 27)]
#[error(E0031, 48, 27)]
#[error(E0031, 50, 4)]
#[error(E0031, 62, 4)]
#[error(E0031, 71, 4)]

package test;

public class E0031 // SpaceInSpace
{
  public single_space LMax a;
  public single_space LMax b;

  public proc test_ko1() = space space nothing end end
  public proc test_ko2() = space
    par
    || space nothing end
    || nothing
    end
  end
  public proc test1() = space nothing end
  public proc test_ko3() = space run test1() end
  public proc test_ko4() =
    module E0031 m = new E0031();
    space run m.test1() end
  end

  public proc test_ko5() = space prune end
  public proc test2() = prune
  public proc test_ko6() = space run test2() end
  public proc test_ko7() =
    space
      module E0031 m = new E0031();
      run m.test2()
    end

  public proc test3() =
    par
    || prune
    || space nothing end
    end

  public proc test_ko8() =
    space run test3() end

  public proc test4() =
    a <- 1;
    prune;
    prune;
  end

  public proc test_ko9() =
    space run test4() end
}
