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

// Example 4.4, Section 4.5.5 in the dissertation (Talbot, 2018).

package test;

public class E0033_example4_4
{
  public proc test1() =
    single_time LMax x = new LMax(0);
    single_time LMax y = new LMax(0);
    when x |= y then
      x <- 2
    else
      y <- 3;
      pause
    end;
    x <- 3;
  end

  public proc test2() =
    single_time LMax x = new LMax(0);
    single_time LMax y = new LMax(0);
    when x |= y then
      x <- 2
    else
      pause;
      y <- 3;
    end;
    x <- 3;
  end

  public proc test3() =
    single_time LMax x = new LMax(0);
    single_time LMax y = new LMax(0);
    when x |= y then
      x <- 2
    else
      pause;
      y <- 3;
    end;
    x <- 3;
  end
}
