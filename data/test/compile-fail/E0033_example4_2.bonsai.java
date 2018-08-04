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

// Example 4.2, Section 4.5.2 in the dissertation (Talbot, 2018).

#[error(E0033, 1, 0)]

package test;

public class E0033_example4_2
{
  public proc test() =
    single_time LMax x = new LMax(0);
    single_time LMax y = new LMax(0);
    par x <- read y.plus(1) || y <- read x.plus(1) end
  end
}
