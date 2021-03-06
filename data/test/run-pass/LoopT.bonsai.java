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

#[run(LoopT.loopOne, "1e")]
#[run(LoopT.loopTwo, "10e")]
#[run(LoopT.loopJoinSpace, "210-1e")]
#[run(LoopT.loopScopeBug, "bot")]
#[run(LoopT.loopInnerDeclaration, "1")]
#[run(LoopT.flowOne, "1e")]
#[run(LoopT.flowTwo, "10e")]
#[run(LoopT.flowProcOneInner, "1e")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;

public class LoopT
{
  public proc loopOne() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      loop
        System.out.print(1);
        pause;
      end;
      System.out.print("unreachable");
    end;
    System.out.print("e");
  end

  public proc loopTwo() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      single_space LMin nodes = new LMin(2);
      loop
        readwrite nodes.dec();
        when nodes |= 0 then
          nothing
        else
          space nothing end
        end;
        System.out.print(read nodes);
        pause;
      end;
      System.out.print("unreachable");
    end;
    System.out.print("e")
  end

  public proc loopJoinSpace() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      single_space LMin nodes = new LMin(2);
      loop
        when nodes |= 0 then
          nothing
        else
          space nothing end
        end;
        System.out.print(read nodes);
        pause;
        readwrite nodes.dec();
        when nodes |= 0 then nothing else space nothing end end;
      end;
      System.out.print("unreachable");
    end;
    System.out.print("e")
  end

  public proc loopScopeBug() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      space nothing end;
      loop
        single_time LMax x = bot;
        x <- 1;
        pause;
        System.out.print(read x);
      end
    end
  end

  public proc loopInnerDeclaration() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      loop
        single_space LMax x = new LMax(0);
        x <- 1;
        space nothing end;
        pause;
        System.out.print(read x);
        when x |= 1 then stop end;
      end
    end
  end

  public proc flowOne() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      flow System.out.print(1) end;
      System.out.print("unreachable");
    end;
    System.out.print("e");
  end

  public proc flowTwo() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      single_space LMin nodes = new LMin(2);
      flow
        readwrite nodes.dec();
        when nodes |= 0 then
          nothing
        else
          space nothing end
        end;
        System.out.print(read nodes);
      end;
      System.out.print("unreachable");
    end;
    System.out.print("e")
  end

  public proc flowProcOneInner() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      run flowProcOne();
      System.out.print("unreachable");
    end;
    System.out.print("e");
  end

  flow flowProcOne() = System.out.print(1)
}
