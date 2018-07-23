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

#[error(E0032, 28, 4)]
#[error(E0032, 32, 19)]
#[error(E0032, 37, 6)]
#[error(E0032, 41, 21)]

package test;

public class E0032
{
  single_space LMax a = bot;

  public proc test1() =
    readwrite a.inc();
    readwrite a.inc();
  end

  public proc test2() =
    f(readwrite a, readwrite a);
  end

  public proc test3() =
    f(readwrite a);
    f(readwrite a);
  end

  public proc test4() =
    f(readwrite a, f(readwrite a))
}
