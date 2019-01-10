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

import java.lang.System;
import java.util.*;
import bonsai.runtime.queueing.*;

public class Simple
{
  single_space LMax nodes = new LMax(0);
  public proc test() =
    single_space LMax depth = new LMax(0);
    par
    <> pause
    <> readwrite nodes.inc()
    end;
    System.out.println(read nodes);
    run test2();
  end

  public proc test2() =
    System.out.println(read nodes);
    pause;
    readwrite nodes.inc();
    System.out.println(read nodes);
  end
}
