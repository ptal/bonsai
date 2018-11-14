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

#[run(DelayT.pauseNothing, "")]
#[run(DelayT.onePausePrint, "1")]
#[run(DelayT.twoPausePrint, "1234")]
#[run(DelayT.pausePrintPause, "1")]
#[run(DelayT.pausePrintPausePrint, "12")]
#[run(DelayT.stopNothing, "")]
#[run(DelayT.pauseUpNothing, "")]
#[run(DelayT.stopStmt, "12")]
#[run(DelayT.pauseUp123, "123")]

package test;

import java.lang.System;
import java.util.*;
import bonsai.runtime.lattices.LMax;

public class DelayT
{
  public proc pauseNothing() = pause; nothing end
  public proc onePausePrint() = pause; System.out.println("1") end
  public proc twoPausePrint() = pause; pause; System.out.println("1234") end

  public proc pausePrintPause() = pause; System.out.print(1); pause; end
  public proc pausePrintPausePrint() =
    pause; System.out.print(1); pause; System.out.print(2); end

  public proc stopNothing() = stop; System.out.print(1) end

  public proc pauseUpNothing() = pause up; System.out.print(1) end

  public proc stopStmt() =
    System.out.print(1);
    pause;
    System.out.print(2);
    stop;
    System.out.print(3);
  end

  public proc pauseUp123() =
    universe
      System.out.print(1);
      pause up;
      System.out.print(2)
    end;
    System.out.print(3)
  end
}
