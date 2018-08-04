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

package test;

public class E0033_9
{
  public proc test() =
    single_space LMax x = new LMax(0);
    single_space LMax y = new LMax(0);
    when x |= y then
      par
      || x <- 2
      || when y |= 3 then x <- 3 end
      || when x |= 3 then x <- 4 end
      || when 4 |= y then x <- 5 end
      end
    else
      par
      || y <- 3
      || when 3 |= x then y <- 1 end
      || when y |= 2 then y <- 1 end
      || when x |= 1 then y <- 4 end
      end
    end
  end
}
