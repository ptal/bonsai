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

#[error(E0033, 1, 0)]
#[error(E0033, 1, 0)]

package test;

/// If we do a more precise causality analysis, taking into account that both entailment are equal, these two examples should succeed.

public class E0033_9
{
  public proc test() =
    single_time LMax x = new LMax(0);
    single_time LMax y = new LMax(0);
    par
    || when x |= y then x <- 1 end
    || when x |= y then x <- 2 else y <- 3 end
    end
  end

  public proc test2() =
    single_time LMax x = new LMax(0);
    single_time LMax y = new LMax(0);
    par
    || when x |= y then nothing else y <- 2 end
    || when x |= y then x <- 2 else y <- 3 end
    end
  end
}
