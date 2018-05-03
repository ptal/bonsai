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

package bonsai.runtime.queueing;

import bonsai.runtime.core.Queueing;
import java.util.*;

public class StackLR<T> implements Queueing<T>
{
  ArrayDeque<T> data;

  public StackLR() {
    data = new ArrayDeque();
  }

  public void push(ArrayList<T> store) {
    for(int i = store.size()-1; i >= 0; i--) {
      data.push(store.get(i));
    }
  }

  public T pop() {
    return data.pop();
  }

  public int size() {
    return data.size();
  }
}
