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

#[run(PruneT.printNothing, "")]
#[run(PruneT.printOne, "1")]
#[run(PruneT.pruneSpacePrune, "12")]
#[run(PruneT.pruneSpacePruneSpacePrune, "1234")]
#[run(PruneT.spacePrune, "12")]
#[run(PruneT.pruneSpace, "12")]
#[run(PruneT.delayedPrune, "12")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;

public class PruneT
{
  public proc printNothing() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      prune
    end
  end

  public proc printOne() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      prune; System.out.print(1); prune
    end
  end

  public proc pruneSpacePrune() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      prune; space System.out.print("1"); end; prune;
      pause;
      System.out.print("2");
      pause;
      System.out.print("unreachable");
    end
  end

  public proc pruneSpacePruneSpacePrune() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      prune; space System.out.print("1"); end; prune;
      space System.out.print("3"); end; prune;
      pause;
      System.out.print("2");
      pause;
      System.out.print("4");
      pause;
      System.out.print("unreachable");
    end
  end

  public proc spacePrune() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      space System.out.print("1"); end; prune;
      pause;
      System.out.print("2");
      pause;
      System.out.print("unreachable");
    end
  end

  public proc pruneSpace() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      prune; space System.out.print("1"); end;
      pause;
      System.out.print("2");
      pause;
      System.out.print("unreachable");
    end
  end

  public proc delayedPrune() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      space System.out.print("1"); end;
      pause;
      prune;
      System.out.print("2");
      pause;
      System.out.print("unreachable");
    end
  end
}