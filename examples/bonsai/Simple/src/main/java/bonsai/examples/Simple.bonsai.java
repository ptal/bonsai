// Copyright 2018 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.examples;

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;
import bonsai.runtime.core.*;

public class Simple
{
  public proc test() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      single_space LMax nodes = new LMax(0);
      loop
        readwrite nodes.inc();
        when nodes |= 2 then
          System.out.print("nodes |= 2");
          nothing
        else
          System.out.print("nodes =| 2");
          space nothing end
        end;
        System.out.print(read nodes);
        pause;
      end;
      System.out.print("unreachable");
    end;
    System.out.print("e")
  end
}
