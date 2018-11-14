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

#[run(Delay.pauseNothing, "")]
#[run(Delay.onePausePrint, "1")]
#[run(Delay.twoPausePrint, "1234")]
#[run(Delay.pausePrintPause, "1")]
#[run(Delay.pausePrintPausePrint, "12")]
#[run(Delay.pauseDecl, "122")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.lattices.LMax;

public class Delay
{
  public proc pauseNothing() = pause; nothing end
  public proc onePausePrint() = pause; System.out.println("1") end
  public proc twoPausePrint() = pause; pause; System.out.println("1234") end

  public proc pausePrintPause() = pause; System.out.print(1); pause; end
  public proc pausePrintPausePrint() =
    pause; System.out.print(1); pause; System.out.print(2); end

  public proc pauseDecl() =
    single_space LMax a = new LMax(1);
    System.out.println(read a);
    pause;
    readwrite a.inc();
    System.out.println(read a);
    pause;
    System.out.println(read a);
  end
}
