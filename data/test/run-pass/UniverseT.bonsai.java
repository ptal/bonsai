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
#[run(UniverseT.twoPrintChild, "123456")]
#[run(UniverseT.successiveChild, "123456")]
#[run(UniverseT.successiveBinaryChild, "11a22a32b41b56")]
#[run(UniverseT.countSpace, "012")]
#[run(UniverseT.countSpace2, "012")]
#[run(UniverseT.sequenceUniverse, "1234")]
#[run(UniverseT.sequenceUniverse2, "12345")]
#[run(UniverseT.nestedUniverse, "123456")]

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

  public proc countSpace() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      single_space LMax count = new LMax(0);
      space readwrite count.inc() end;
      space readwrite count.inc() end;
      System.out.print(read count);
      pause;
      System.out.print(read count);
      pause;
      System.out.print(read count);
    end
  end

  public proc countSpace2() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      single_space LMax count = new LMax(0);
      space readwrite count.inc() end;
      System.out.print(read count);
      pause;
      space readwrite count.inc() end;
      System.out.print(read count);
      pause;
      System.out.print(read count);
    end
  end

  public proc sequenceUniverse() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      System.out.print(1);
      space nothing end;
      pause;
      System.out.print(2);
    end;
    pause;
    universe with stack in
      System.out.print(3);
      space nothing end;
      pause;
      System.out.print(4);
    end
  end

  public proc sequenceUniverse2() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      System.out.print(1);
      space nothing end;
      pause;
      System.out.print(2);
      space nothing end;
      space nothing end;
    end;
    pause;
    universe with stack in
      System.out.print(3);
      pause;
      System.out.print(4);
      pause;
      System.out.print(5);
    end
  end


  public proc nestedUniverse() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      System.out.print(1);
      space nothing end;
      single_space StackLR stack2 = new StackLR();
      universe with stack2 in
        System.out.print(2);
        space System.out.print(3) end;
        pause;
        pause;
      end;
      System.out.print(4);
      pause;
      System.out.print(5);
    end
    System.out.print(6)
  end
}
