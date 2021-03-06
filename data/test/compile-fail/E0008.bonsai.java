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

#[error(E0008, 29, 6)]
#[error(E0008, 30, 20)]
#[error(E0008, 34, 20)]

package test;
import test.Module;
import test.J;

public class UnknownField
{
  module Module m = new Module();

  public proc test() =
    m.a;
    m.c;
    J.inside_args(m.d, m.a);
  end

  public proc test2() =
    universe with m.c in nothing end
}
