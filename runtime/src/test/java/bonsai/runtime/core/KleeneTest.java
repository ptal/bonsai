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

import static org.junit.Assert.*;
import static org.hamcrest.CoreMatchers.*;
import org.junit.*;

public class KleeneTest
{
  @Test
  public void testNot() {
    assertThat(Kleene.not(Kleene.TRUE), equalTo(Kleene.FALSE));
    assertThat(Kleene.not(Kleene.FALSE), equalTo(Kleene.TRUE));
    assertThat(Kleene.not(Kleene.UNKNOWN), equalTo(Kleene.UNKNOWN));
  }

  @Test
  public void testAnd() {
    assertThat(Kleene.and(Kleene.TRUE, Kleene.TRUE), equalTo(Kleene.TRUE));
    assertThat(Kleene.and(Kleene.FALSE, Kleene.TRUE), equalTo(Kleene.FALSE));
    assertThat(Kleene.and(Kleene.TRUE, Kleene.FALSE), equalTo(Kleene.FALSE));
    assertThat(Kleene.and(Kleene.FALSE, Kleene.FALSE), equalTo(Kleene.FALSE));
    assertThat(Kleene.and(Kleene.UNKNOWN, Kleene.FALSE), equalTo(Kleene.FALSE));
    assertThat(Kleene.and(Kleene.FALSE, Kleene.UNKNOWN), equalTo(Kleene.FALSE));
    assertThat(Kleene.and(Kleene.UNKNOWN, Kleene.UNKNOWN), equalTo(Kleene.UNKNOWN));
    assertThat(Kleene.and(Kleene.TRUE, Kleene.UNKNOWN), equalTo(Kleene.UNKNOWN));
    assertThat(Kleene.and(Kleene.UNKNOWN, Kleene.TRUE), equalTo(Kleene.UNKNOWN));
  }

  @Test
  public void testOr() {
    assertThat(Kleene.or(Kleene.TRUE, Kleene.TRUE), equalTo(Kleene.TRUE));
    assertThat(Kleene.or(Kleene.FALSE, Kleene.TRUE), equalTo(Kleene.TRUE));
    assertThat(Kleene.or(Kleene.TRUE, Kleene.FALSE), equalTo(Kleene.TRUE));
    assertThat(Kleene.or(Kleene.FALSE, Kleene.FALSE), equalTo(Kleene.FALSE));
    assertThat(Kleene.or(Kleene.UNKNOWN, Kleene.FALSE), equalTo(Kleene.UNKNOWN));
    assertThat(Kleene.or(Kleene.FALSE, Kleene.UNKNOWN), equalTo(Kleene.UNKNOWN));
    assertThat(Kleene.or(Kleene.UNKNOWN, Kleene.UNKNOWN), equalTo(Kleene.UNKNOWN));
    assertThat(Kleene.or(Kleene.TRUE, Kleene.UNKNOWN), equalTo(Kleene.TRUE));
    assertThat(Kleene.or(Kleene.UNKNOWN, Kleene.TRUE), equalTo(Kleene.TRUE));
  }

  @Test
  public void testFromBool() {
    assertThat(Kleene.fromBool(true), equalTo(Kleene.TRUE));
    assertThat(Kleene.fromBool(false), equalTo(Kleene.FALSE));
  }
}
