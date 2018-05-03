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

package bonsai.runtime.core;

import java.util.*;
import bonsai.runtime.queueing.*;
import bonsai.runtime.core.*;
import static org.junit.Assert.*;
import static org.hamcrest.CoreMatchers.*;
import org.junit.*;

public class QueueingTest
{
  protected String currentTest;

  public void pushRoot(Queueing<Integer> queue, Integer x) {
    assert queue.size() == 0;
    ArrayList<Integer> store = new ArrayList<>(Arrays.asList(new Integer[]{x}));
    queue.push(store);
    Integer x2 = queue.pop();
    assertThat(currentTest, x, equalTo(x2));
    assertThat(currentTest, queue.size(), equalTo(0));
    queue.push(store);
  }

  public void pushPopTest(int width, int from, Queueing<Integer> queue, Integer expected) {
    Integer popped = queue.pop();
    assertThat(currentTest, popped, equalTo(expected));
    ArrayList<Integer> store = new ArrayList();
    for(int i = 0; i < width; i++) {
      store.add(from + i);
    }
    queue.push(store);
  }
}
