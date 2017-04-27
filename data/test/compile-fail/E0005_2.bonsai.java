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

#[error(E0005, 24, 28)]
#[error(E0005, 24, 37)]

package test;

public class IllegalInitialization
{
  single_space T mref;
  module Module2 t1;
  single_space T t2 = new T(t1.mref, t1.m);

  proc test() { nothing; }
}
