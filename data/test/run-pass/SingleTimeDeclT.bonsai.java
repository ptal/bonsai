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
#[run(SingleTimeDeclT.oneInstant, "12")]
#[run(SingleTimeDeclT.severalInstant, "121")]
#[run(SingleTimeDeclT.pauseUpInUniverse, "21")]
#[run(SingleTimeDeclT.reinitValue, "012")]
#[run(SingleTimeDeclT.scopeBug, "2")]
#[run(SingleTimeDeclT.initSingleTimeBug, "1")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.lattices.LMax;
import bonsai.runtime.queueing.*;

public class SingleTimeDeclT
{
  public proc printBottom() =
    single_time LMax a;
    System.out.print(a);
  end

  public proc printBottom2() =
    single_time LMax a = bot;
    System.out.print(a);
  end

  public proc printOne() =
    single_time LMax a = new LMax(1);
    System.out.print(a);
  end

  public proc printTwo() =
    single_time LMax a = new LMax(1);
    readwrite a.inc();
    System.out.print(read a);
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

  public proc scopeBug() =
    single_time ES consistent = unknown;
    consistent <- true;
    loop
      single_time ES unk = unknown;
      when unk |= consistent then
        stop
      else
        System.out.print(2);
      end;
      pause;
    end
  end

  public proc initSingleTimeBug() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      single_time LMax a = new LMax(1);
      space nothing end;
      pause;
      a <- 3;
      System.out.print("1");
    end
  end

  static int i = 0;
  private static LMax init() {
    return new LMax(i++);
  }
}
