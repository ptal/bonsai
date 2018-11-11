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

#[run(NothingT.test, "")]
#[run(NothingT.printOne, "1")]
#[run(NothingT.printFew, "1234")]
#[run(NothingT.printFew2, "1+")]
#[run(NothingT.print41, "41")]

package test;

import java.lang.System;
import java.util.*;

public class NothingT
{
  public proc test() = nothing
  public proc printOne() = System.out.println("1")
  public proc printFew() = System.out.println("1234")
  public proc printFew2() = System.out.println("1111111")
  public proc print41() = System.out.println(41)
}
