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

public class StackTest extends QueueingTest
{
  @Test
  public void testStackLR() {
    //        0
    //     1     2
    //   3   4 5   6
    currentTest = "StackLR";
    StackLR<Integer> stack = new StackLR<>();
    pushRoot(stack, 0);              // [0]
    pushPopTest(2, 1, stack, 0);     // [2, 1]
    pushPopTest(2, 3, stack, 1);     // [2, 4, 3]
    assertThat(currentTest, stack.pop(), equalTo(3)); // [2, 4]
    assertThat(currentTest, stack.pop(), equalTo(4)); // [2]
    pushPopTest(2, 5, stack, 2);     // [6, 5]
    assertThat(currentTest, stack.pop(), equalTo(5)); // [6]
    assertThat(currentTest, stack.pop(), equalTo(6)); // []
    assertThat(currentTest, stack.size(), equalTo(0));
  }

  @Test
  public void testStackRL() {
    //        0
    //     1     2
    //   3   4 5   6
    currentTest = "StackRL";
    StackRL<Integer> stack = new StackRL<>();
    pushRoot(stack, 0);              // [0]
    pushPopTest(2, 1, stack, 0);     // [1, 2]
    pushPopTest(2, 5, stack, 2);     // [1, 5, 6]
    assertThat(currentTest, stack.pop(), equalTo(6)); // [1, 5]
    assertThat(currentTest, stack.pop(), equalTo(5)); // [1]
    pushPopTest(2, 3, stack, 1);     // [3, 4]
    assertThat(currentTest, stack.pop(), equalTo(4)); // [4]
    assertThat(currentTest, stack.pop(), equalTo(3)); // []
    assertThat(currentTest, stack.size(), equalTo(0));
  }
}
