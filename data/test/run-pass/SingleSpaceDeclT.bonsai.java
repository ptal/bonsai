// Copyright 2018 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[run(SingleSpaceDeclT.printBottom, "bot")]
#[run(SingleSpaceDeclT.printBottom2, "bot")]
#[run(SingleSpaceDeclT.printOne, "1")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.lattices.LMax;

public class SingleSpaceDeclT
{
  public proc printBottom() =
    single_space LMax a;
    System.out.println(a);
  end

  public proc printBottom2() =
    single_space LMax a = bot;
    System.out.println(a);
  end

  public proc printOne() =
    single_space LMax a = new LMax(1);
    System.out.println(a);
  end
}
