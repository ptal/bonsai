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

#[run(SingleTimeDeclT.printBottom, "bot")]
#[run(SingleTimeDeclT.printBottom2, "bot")]
#[run(SingleTimeDeclT.printOne, "1")]
#[run(SingleTimeDeclT.printTwo, "2")]
#[debug(SingleTimeDeclT.oneInstant, "12")]
#[debug(SingleTimeDeclT.severalInstant, "121")]
#[debug(SingleTimeDeclT.pauseUpInUniverse, "21")]
#[debug(SingleTimeDeclT.reinitValue, "012")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.lattices.LMax;

public class SingleTimeDeclT
{
  public proc printBottom() =
    single_time LMax a;
    System.out.println(a);
  end

  public proc printBottom2() =
    single_time LMax a = bot;
    System.out.println(a);
  end

  public proc printOne() =
    single_time LMax a = new LMax(1);
    System.out.println(a);
  end

  public proc printTwo() =
    single_time LMax a = new LMax(1);
    readwrite a.inc();
    System.out.println(read a);
  end

  public proc oneInstant() =
    single_time LMax a = new LMax(1);
    System.out.print(read a);
    pause;
    readwrite a.inc();
    System.out.print(read a);
  end

  public proc severalInstant() =
    single_time LMax a = new LMax(1);
    System.out.print(read a);
    pause;
    readwrite a.inc();
    System.out.print(read a);
    pause;
    System.out.print(read a);
  end

  public proc pauseUpInUniverse() =
    single_time LMax a = new LMax(1);
    readwrite a.inc();
    System.out.print(read a);
    universe
      pause up
    end;
    System.out.print(read a);
  end

  public proc reinitValue() =
    single_time LMax a = init();
    System.out.print(read a);
    pause;
    System.out.print(read a);
    pause;
    System.out.print(read a);
  end

  static int i = 0;
  private static LMax init() {
    return new LMax(i++);
  }
}
