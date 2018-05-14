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

#[error(E0028, 34, 20)]
#[error(E0028, 37, 4)]
#[error(E0028, 42, 4)]
#[error(E0028, 52, 4)]
#[error(E0028, 59, 4)]
#[error(E0028, 66, 4)]
#[error(E0028, 79, 8)]
#[error(E0028, 89, 8)]
#[error(E0028, 87, 4)]
#[error(E0028, 101, 4)]
#[error(E0028, 106, 4)]

package test;

public class E0028 // InstantaneousLoop
{
  public single_space LMax a;
  public single_space LMax b;

  proc test_ko1() = loop nothing end

  proc test_ko2() =
    loop
      when a |= b then nothing end
    end

  proc test_ko3() =
    loop
      when a |= b then
        nothing
      else
        loop pause end
      end;
      a <- f(a)
    end

  proc test_ko4() =
    loop
      suspend when a |= b in
        nothing
      end
    end

  proc test_ko5() =
    loop
      abort when a |= b in
        pause
      end
    end

  proc test_ko6() =
    loop
      when a |= b then
        suspend when a |= b in
          loop pause end
        end
      else
        when a |= b then pause end
      end
    end

  proc test_ko7() =
    loop
      abort when a |= b in
        loop
          when a |= b then pause end
        end
      end;
      pause
    end

  proc test_ko8() =
    loop
      abort when a |= b in
        loop
          when a |= b then pause end
        end
      end
    end

  proc test1() =
    abort when a |= b in
      pause
    end

  proc test_ko9() =
    loop
      run test1();
    end

  proc test_ko10() =
    loop
      par
      || run test1()
      || run test1()
      end
    end
}
