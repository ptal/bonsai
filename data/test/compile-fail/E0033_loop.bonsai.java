// Copyright 2019 Pierre Talbot

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

public class E0033_loop
{
  public proc test() =
    loop
      single_time LMax x = new LMax(0);
      single_time LMax y = new LMax(0);
      when y |= x then x <- 1 end;
      pause
    end
  end

  public proc test2() =
    single_time LMax x = new LMax(0);
    loop
      single_time LMax y = new LMax(0);
      x <- 1;
      pause;
      when y |= x then y <- 1 end;
    end
  end
}
