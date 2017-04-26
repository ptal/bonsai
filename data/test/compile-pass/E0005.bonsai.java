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

package test;

public class CorrectFieldInitialization
{
  single_space Integer t1 = new Integer(0);
  single_space Boolean t2 = new Boolean(true);
  single_space Mixed t3 = new Mixed(0, true, 8, "bonsai");

  proc test() { nothing; }
}
