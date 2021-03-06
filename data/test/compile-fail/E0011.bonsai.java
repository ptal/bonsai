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

#[error(E0011, 21, 2)]

package test;

public class IllegalRefInit
{
  ref single_space T a = new T();
  single_space N b;
  ref single_space T ok;

  public IllegalRefInit(T a, T ok) {
    this.a = a;
    this.ok = ok;
  }
}
