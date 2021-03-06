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

public class E0033_nonreactive
{
  public proc test() =
    universe
      single_time LMax x = new LMax(0);
      when x |= 1 then nothing else x <- 1 end
    end

  public proc test2() =
    single_space Queue q = new Queue();
    universe with q in
      single_time LMax x = new LMax(0);
      when x |= 1 then nothing else x <- 1 end
    end
  end
}
