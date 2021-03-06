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

import java.lang.System;

public class E0006
{
  public proc test() =
    System.out.println("test");
  end

  public proc test2() =
    universe
      single_space LMax a = new LMax(0);
      pause;
      System.out.print(read a);
    end

  public proc test3() =
    single_space LMax a = bot;
    a <- 1;
    par
    || single_time ES unk = unknown;
       unk |= true;
    end
  end
}
