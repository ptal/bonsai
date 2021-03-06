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

#[error(E0012, 30, 2)]

package test;

public class MultipleConstructor
{
  ref single_space T a;
  single_space N b;
  ref single_space T ok;

  public MultipleConstructor(T a, T ok, int i) {
    this(a, ok);
    this.b = new N(i);
  }

  public MultipleConstructor(T a, T ok) {
    this.a = a;
    this.ok = ok;
  }

  public MultipleConstructor(T a, T ok, N b) {
    this.a = a;
    this.ok = ok;
    this.b = b;
  }
}
