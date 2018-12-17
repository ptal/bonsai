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

#[run(ParallelT.printNothing, "")]
#[run(ParallelT.printNothing2, "")]
#[run(ParallelT.interleaving, "11")]
#[run(ParallelT.interleaving2, "11")]
#[run(ParallelT.interleavingPause, "01")]
#[run(ParallelT.interleavingPause2, "00")]
#[run(ParallelT.example3_2_thesis, "13")]
#[run(ParallelT.spaceSpace, "12")]
#[run(ParallelT.spacePrune, "1")]
#[run(ParallelT.pruneSpace, "1")]
#[run(ParallelT.prunePrune, "")]
#[run(ParallelT.spaceSpace2, "12")]
#[run(ParallelT.spacePrune2, "")]
#[run(ParallelT.pruneSpace2, "")]
#[run(ParallelT.prunePrune2, "")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;

public class ParallelT
{
  public proc printNothing() = par nothing <> nothing end
  public proc printNothing2() = par nothing || nothing end

  public proc interleaving() =
    single_time LMax x = new LMax(0);
    par System.out.print(read x) || readwrite x.inc() end;
    System.out.print(read x);
  end

  public proc interleaving2() =
    single_time LMax x = new LMax(0);
    par System.out.print(read x) <> readwrite x.inc() end;
    System.out.print(read x);
  end

  public proc interleavingPause() =
    single_space LMax x = new LMax(0);
    par System.out.print(read x) || pause; readwrite x.inc() end;
    System.out.print(read x);
  end

  public proc interleavingPause2() =
    single_space LMax x = new LMax(0);
    par System.out.print(read x) <> pause; readwrite x.inc() end;
    System.out.print(read x);
  end

  public proc example3_2_thesis() =
    single_time LMax x = new LMax(1);
    single_time LMax y = new LMax(0);
    par
    || when x |= y then x <- 2 else y <- 3 end
    || y <- 2
    || System.out.print(read x);
       System.out.print(read y);
    end
  end

  public proc spaceSpace() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par space System.out.print(1) end
      ||  space System.out.print(2) end
      end;
      pause;
      pause;
      System.out.print("unreachable");
    end
  end

  public proc spacePrune() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par space System.out.print(1) end
      ||  prune
      end;
      pause;
      pause;
      System.out.println("unreachable");
    end
  end

  public proc pruneSpace() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par prune
      ||  space System.out.print(1) end
      end;
      pause;
      pause;
      System.out.print("unreachable");
    end
  end

  public proc prunePrune() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par prune
      ||  prune
      end;
      pause;
      System.out.print("unreachable");
    end
  end

  public proc spaceSpace2() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par space System.out.print(1) end
      <>  space System.out.print(2) end
      end;
      pause;
      pause;
      System.out.print("unreachable");
    end
  end

  public proc spacePrune2() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par space System.out.print(1) end
      <>  prune
      end;
      pause;
      System.out.print("unreachable");
    end
  end

  public proc pruneSpace2() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par prune
      <>  space System.out.print(1) end
      end;
      pause;
      System.out.print("unreachable");
    end
  end

  public proc prunePrune2() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      par prune
      <>  prune
      end;
      pause;
      System.out.print("unreachable");
    end
  end
}
