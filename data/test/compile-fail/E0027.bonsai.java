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

#[error(E0027, 32, 9)]
#[error(E0027, 33, 14)]
#[error(E0027, 35, 9)]
#[error(E0027, 36, 22)]
#[error(E0027, 37, 30)]
#[error(E0027, 38, 9)]

package test;

public class IllegalHostFunctionInReadContext
{
  public single_space LMax a;
  public single_space LMax b;
  public ES t;

  public proc test() =
    t <- a |= b; // OK
    t <- f(a) |= b; // KO
    t <- a |= f(b); // KO
    t <- f(a |= b); // OK
    when f(a) |= b then nothing end; // KO
    suspend when a |= f(b) in nothing end; // KO
    abort when not(t or (a |= f(b))) in nothing end; // KO
    when f() then nothing end; // KO
  end
}
