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

#[run(UniverseT.printNothing, "")]
#[run(UniverseT.printNothing2, "")]
#[run(UniverseT.oneChildImmediate, "")]
#[run(UniverseT.oneNothingChild, "1")]
#[run(UniverseT.onePrintChild, "12")]
#[run(UniverseT.onePrintChild2, "1234")]
#[debug(UniverseT.twoPrintChild, "123456")]
#[debug(UniverseT.successiveChild, "123456")]
#[debug(UniverseT.successiveBinaryChild, "11a22a32b41b56")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;

public class UniverseT
{
  public proc printNothing() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      nothing
    end
  end

  public proc printNothing2() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      pause;
      System.out.print("unreachable");
    end
  end

  public proc oneChildImmediate() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      space nothing end;
    end
  end

  public proc oneNothingChild() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      space nothing end;
      pause;
      System.out.print("1");
    end
  end


  public proc onePrintChild() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      space System.out.print("2") end;
      System.out.print("1");
      pause;
    end
  end

  public proc onePrintChild2() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      space System.out.print("2") end;
      System.out.print("1");
      pause;
      System.out.print("3");
      pause;
      System.out.print("unreachable");
    end;
    System.out.print("4")
  end

  public proc twoPrintChild() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      space System.out.print("2") end;
      space System.out.print("4") end;
      System.out.print("1");
      pause;
      System.out.print("3");
      pause;
      System.out.print("5");
      pause;
      System.out.print("unreachable");
    end;
    System.out.print("6")
  end

  public proc successiveChild() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      space System.out.print("2") end;
      System.out.print("1");
      pause;
      space System.out.print("4") end;
      System.out.print("3");
      pause;
      System.out.print("5");
      pause;
      System.out.print("unreachable");
    end;
    System.out.print("6")
  end

  public proc successiveBinaryChild() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      space System.out.print("1a") end;
      space System.out.print("1b") end;
      System.out.print("1");
      pause;
      space System.out.print("2a") end;
      space System.out.print("2b") end;
      System.out.print("2");
      pause;
      System.out.print("3");
      pause;
      System.out.print("4");
      pause;
      System.out.print("5");
      pause;
      System.out.print("unreachable");
    end;
    System.out.print("6")
  end
}
