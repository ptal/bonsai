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

#[error(E0026, 32, 4)]
#[error(E0026, 33, 4)]
#[error(E0026, 38, 14)]
#[error(E0026, 39, 9)]
#[error(E0026, 39, 20)]
#[error(E0026, 40, 9)]

package test;

public class IllegalPermissionInContext
{
  public single_space LMax a;
  public single_space LMax b;

  proc test() =
    write a <- 1; // OK
    a <- 2; // OK
    read a <- 3; // KO
    readwrite a <- 4; // KO
    when a |= b then nothing end; // OK
    when read a |= b then nothing end; // OK
    when a |= read b then nothing end; // OK
    when read a |= read b then nothing end; // OK
    when a |= write b then nothing end; // KO
    when write a |= write b then nothing end; // KO
    when readwrite a |= b then nothing end; // KO
  end
}
