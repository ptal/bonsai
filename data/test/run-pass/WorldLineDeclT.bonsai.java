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

#[run(WorldLineDeclT.rootNode, "0")]
#[run(WorldLineDeclT.oneBranchTree, "00")]
#[run(WorldLineDeclT.twoBranchesTree, "000")]
#[run(WorldLineDeclT.twoBranchesTree2, "011")]
#[run(WorldLineDeclT.init, "0")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;
import bonsai.runtime.lattices.*;

public class WorldLineDeclT
{
  public proc rootNode() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      world_line LMax count = new LMax(0);
      System.out.print(count);
      pause;
      System.out.print("unreachable");
    end
  end

  public proc oneBranchTree() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      world_line LMax count = new LMax(0);
      space nothing end;
      System.out.print(count);
      pause;
      System.out.print(count);
      pause;
      System.out.print("unreachable");
    end
  end

  public proc twoBranchesTree() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      world_line LMax count = new LMax(0);
      space nothing end;
      space nothing end;
      System.out.print(count);
      pause;
      System.out.print(count);
      pause;
      System.out.print(count);
      pause;
      System.out.print("unreachable");
    end
  end

  public proc twoBranchesTree2() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      world_line LMax count = new LMax(0);
      space readwrite count.inc() end;
      space readwrite count.inc() end;
      System.out.print(count);
      pause;
      System.out.print(count);
      pause;
      System.out.print(count);
      pause;
      System.out.print("unreachable");
    end
  end

  public proc init() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      world_line LMax a = new LMax(0);
      System.out.print(read a);
    end
  end
}
