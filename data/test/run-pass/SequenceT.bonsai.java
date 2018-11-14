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

#[run(SequenceT.print123, "123")]
#[run(SequenceT.print123bis, "123")]

package test;

import java.lang.System;
import java.util.*;

public class SequenceT
{
  public proc print123() =
    System.out.print("1");
    System.out.print("2");
    System.out.print("3")
  end

  public proc print123bis() =
    nothing; System.out.print(1);
    nothing; System.out.print(2);
    nothing; System.out.print(3);
    nothing
  end
}
