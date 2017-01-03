// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.runtime.core;

// A store is a lattice-based variable with two additional methods: `alloc` and `index` where:
//  * `alloc` allocates an object in the store and returns its location.
//  * `index` retrieve an object from a location in the store.

public abstract class Store extends LatticeVar {
  public abstract Object alloc(Object value);
  public abstract Object index(Object location);
}
