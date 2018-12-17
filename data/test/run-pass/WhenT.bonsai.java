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

#[run(WhenT.whenTrue, "1")]
#[run(WhenT.whenFalse, "2")]
#[run(WhenT.whenUnknown, "2")]
#[run(WhenT.whenTrueEntailsUnknown, "1")]
#[run(WhenT.whenTrueEntailsFalse, "1")]
#[run(WhenT.whenFalseEntailsFalse, "1")]
#[run(WhenT.whenLMax, "12")]
#[run(WhenT.whenIdentity, "1")]
#[run(WhenT.whenLMax2, "12")]
#[run(WhenT.monotone, "4")]
#[run(WhenT.antiMonotone, "5")]
#[run(WhenT.nestedWhen, "34")]
#[run(WhenT.whenAndTell, "94")]
#[run(WhenT.nonMonotonicTell, "4")]
#[run(WhenT.nonMonotonicTell2, "344")]
#[debug(WhenT.example4_4_thesis, "33")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;

public class WhenT
{
  public proc whenTrue() =
    when true then
      System.out.print(1);
    else
      System.out.print(2);
    end

  public proc whenFalse() =
    when false then
      System.out.print(1);
    else
      System.out.print(2);
    end

  public proc whenUnknown() =
    when unknown then
      System.out.print(1);
    else
      System.out.print(2);
    end

  public proc whenTrueEntailsUnknown() =
    when true |= unknown then
      System.out.print(1);
    end

  public proc whenTrueEntailsFalse() =
    when true |= false then nothing else
      System.out.print(1);
    end

  public proc whenFalseEntailsFalse() =
    when false |= false then
      System.out.print(1);
    end

  public proc whenLMax() =
    single_time LMax x = new LMax(3);
    single_time LMax c2 = new LMax(2);
    when x |= 2 then
      System.out.print(1);
    end;
    when c2 |= x then nothing else
      System.out.print(2);
    end
  end

  public proc whenIdentity() =
    single_time LMax x = new LMax(3);
    when x |= x then
      System.out.print(1);
    end
  end

  public proc whenLMax2() =
    single_time LMax x = new LMax(3);
    single_time LMax y = new LMax(2);
    when x |= y then
      System.out.print(1);
    end;
    when y |= x then nothing else
      System.out.print(2);
    end
  end

  public proc monotone() =
    single_time LMax x = new LMax(3);
    single_time LMax y = new LMax(2);
    when x |= y then
      readwrite x.inc();
      System.out.print(read x);
    else
      readwrite y.inc();
      System.out.print(read y);
    end
  end

  public proc antiMonotone() =
    single_time LMax x = new LMax(3);
    single_time LMax y = new LMax(4);
    when x |= y then
      readwrite x.inc();
      System.out.print(read x);
    else
      readwrite y.inc();
      System.out.print(read y);
    end
  end

  public proc nestedWhen() =
    single_time StackRL s = new StackRL();
    universe with s in
      single_space LMax x = new LMax(3);
      single_space LMax y = new LMax(3);
      when x |= y then
        space readwrite y.inc() end
      end;
      System.out.print(read y);
      pause;
      when x |= y then
        System.out.print("unreachable");
      else
        System.out.print(read y);
      end
    end
  end

  public proc whenAndTell() =
    single_time LMax x = new LMax(3);
    single_time LMax y = new LMax(4);
    x <- 4;
    when x |= y then
      x <- 9;
    else
      y <- 9;
    end;
    System.out.print(read x);
    System.out.print(read y);
  end

  public proc nonMonotonicTell() =
    single_time LMax x = bot;
    x <- 4;
    x <- 3;
    System.out.print(read x);
  end

  public proc nonMonotonicTell2() =
    single_space LMax x = new LMax(3);
    System.out.print(read x);
    pause;
    x <- 4;
    System.out.print(read x);
    pause;
    x <- 3;
    System.out.print(read x);
  end

  public proc example4_4_thesis() =
    single_space LMax x = bot;
    single_space LMax y = new LMax(1);
    when x |= y then
      x <- 2
    else
      y <- 3;
      pause
    end;
    x <- 3;
    System.out.print(read x);
    System.out.print(read y);
  end
}
