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

#[run(QFUniverseT.printNothing, "")]
#[run(QFUniverseT.printOne, "1")]
#[run(QFUniverseT.printTwo, "2")]

package test;

import java.lang.System;
import java.util.*;

public class QFUniverseT
{
  public proc printNothing() = universe nothing end
  public proc printOne() = universe System.out.print("1") end
  public proc printTwo() = universe universe System.out.print("2") end end
}
