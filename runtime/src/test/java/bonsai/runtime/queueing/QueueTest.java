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

import bonsai.runtime.core.*;
import static org.junit.Assert.*;
import static org.hamcrest.CoreMatchers.*;
import org.junit.*;

public class QueueTest extends QueueingTest
{
  @Test
  public void testQueueLR() {
    //        0
    //     1     2
    //   3   4 5   6
    currentTest = "QueueLR";
    QueueLR<Integer> queue = new QueueLR<>();
    pushRoot(queue, 0);              // [0]
    pushPopTest(2, 1, queue, 0);     // [1, 2]
    pushPopTest(2, 3, queue, 1);     // [2, 3, 4]
    pushPopTest(2, 5, queue, 2);     // [3, 4, 5, 6]
    assertThat(currentTest, queue.pop(), equalTo(3)); // [4, 5, 6]
    assertThat(currentTest, queue.pop(), equalTo(4)); // [5, 6]
    assertThat(currentTest, queue.pop(), equalTo(5)); // [6]
    assertThat(currentTest, queue.pop(), equalTo(6)); // []
    assertThat(currentTest, queue.size(), equalTo(0));
  }

  @Test
  public void testQueueRL() {
    //        0
    //     1     2
    //   3   4 5   6
    currentTest = "QueueRL";
    QueueRL<Integer> queue = new QueueRL<>();
    pushRoot(queue, 0);              // [0]
    pushPopTest(2, 1, queue, 0);     // [2, 1]
    pushPopTest(2, 5, queue, 2);     // [1, 6, 5]
    pushPopTest(2, 3, queue, 1);     // [6, 5, 4, 3]
    assertThat(currentTest, queue.pop(), equalTo(6)); // [5, 4, 3]
    assertThat(currentTest, queue.pop(), equalTo(5)); // [4, 3]
    assertThat(currentTest, queue.pop(), equalTo(4)); // [3]
    assertThat(currentTest, queue.pop(), equalTo(3)); // []
    assertThat(currentTest, queue.size(), equalTo(0));
  }
}
