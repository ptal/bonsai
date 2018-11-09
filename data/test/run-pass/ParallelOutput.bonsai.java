// Copyright 2017 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// When optimization or re-ordering of the parallel branches occurs, the regex should become "123|132|213|231|321|312".
// #[run(test, "123")]

package test;

import java.lang.System;

public class ParallelOutput
{
  public proc test() =
    par
    || System.out.print("1")
    || System.out.print("2")
    || System.out.print("3")
    end
  end
}
