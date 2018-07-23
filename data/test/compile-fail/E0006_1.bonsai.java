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

#[error(E0006, 24, 28)]
#[error(E0006, 32, 43)]

package test;
import java.lang.System;

public class UndeclaredVariable
{
  single_space T t1 = new T();
  single_space T t2 = new T(t3);
  single_space T t3 = new T();

  public proc test() =
    single_space T t4 = new T();
    System.out.println(t4);
  end

  public proc test2() = System.out.println(t4)
}
