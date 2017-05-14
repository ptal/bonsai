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

#[error(E0005, 24, 34)]
#[error(E0005, 24, 42)]

package test;

public class FieldInitializationWithRef
{
  ref single_space T a;
  module Module t1 = new T();
  single_space T t2 = new T(t1.a, t1.b.a, a);

  public FieldInitializationWithRef(T a) {}

  proc test() { nothing; }
}
