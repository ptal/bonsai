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

#[run(SeqUnivDelay.pauseNothing, "")]
#[run(SeqUnivDelay.onePausePrint, "1")]
#[run(SeqUnivDelay.delay3, "12")]
#[run(SeqUnivDelay.print123321, "123321")]
#[run(SeqUnivDelay.print1234321, "1234321")]
#[run(SeqUnivDelay.print1233Stop, "1233")]

package test;

import java.lang.System;
import java.util.*;

public class SeqUnivDelay
{
  public proc pauseNothing() = universe pause; nothing end
  public proc onePausePrint() = pause; universe System.out.print("1") end end
  public proc delay3() =
    pause; System.out.print("1");
    universe pause; System.out.print("2") end;
    pause
  end

  public proc print123321() =
    System.out.print(1);
    universe
      System.out.print(2);
      universe
        System.out.print(3);
      end;
      pause;
      universe
        System.out.print(3);
      end;
      System.out.print(2);
    end;
    System.out.print(1);
  end

  public proc print1234321() =
    System.out.print(1);
    universe
      System.out.print(2);
      universe
        System.out.print(3);
      end;
      pause;
      System.out.print(4);
      universe
        System.out.print(3);
      end;
      System.out.print(2);
    end;
    System.out.print(1);
  end

  public proc print1233Stop() =
    System.out.print(1);
    universe
      System.out.print(2);
      universe
        System.out.print(3);
      end;
      pause;
      universe
        System.out.print(3);
        stop;
      end;
      System.out.print(2);
    end;
    System.out.print(1);
  end
}
