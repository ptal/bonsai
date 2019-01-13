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

import bonsai.statistics.*;

public class HelloWorld
{
  public proc hello() =
    single_space StackLR stack = new StackLR();
    universe with stack in
      module Node nodes = new Node();
      par
      || run sayHello();
      || run nodes.count();
      || flow System.out.println(nodes.value) end
      end
    end
  end

  proc sayHello() =
    single_space LMax sayHello = new LMax(0);
    single_space LMax two = new LMax(4);
    loop
      readwrite sayHello.inc();
      when two |= sayHello then
        System.out.println("Hello world!");
        space nothing end
      end;
      pause
    end
  end
}
