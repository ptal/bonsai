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

package test;

public class NonInstantaneousLoop
{
  single_space LMax a;
  single_space LMax b;

  proc test1() = loop pause end
  proc test2() =
    loop
      suspend when a |= b in pause end
    end
  proc test3() = loop stop end
  proc test4() = loop pause up end
  proc test5() =
    loop
      abort when a |= b in nothing end;
      pause
    end
  proc test6() =
    loop
      loop pause end
    end
  proc test7() =
    loop
      when a |= b then pause else stop end
    end
  proc test8() =
    loop
      when a |= b then
        pause
      else
        loop
          pause
        end
      end
    end

  proc test9() =
    loop
      par
      || when a |= b then stop end
      || pause
      end
    end

  proc test10() =
    loop
      par
      <> when a |= b then pause end
      <> pause up
      end
    end
}
