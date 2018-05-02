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

public class ESTest
{
  ES etrue;
  ES efalse;
  ES eunknown;

  @Before
  public void initES() {
    etrue = new ES(Kleene.TRUE);
    efalse = new ES(Kleene.FALSE);
    eunknown = new ES(Kleene.UNKNOWN);
  }

  @Test
  public void testJoin() {
    assertThat(etrue.join(etrue), equalTo(etrue));
    assertThat(efalse.join(etrue), equalTo(efalse));
    assertThat(etrue.join(efalse), equalTo(efalse));
    assertThat(efalse.join(efalse), equalTo(efalse));
    assertThat(eunknown.join(etrue), equalTo(etrue));
    assertThat(etrue.join(eunknown), equalTo(etrue));
    assertThat(eunknown.join(eunknown), equalTo(eunknown));
    assertThat(efalse.join(eunknown), equalTo(efalse));
    assertThat(eunknown.join(efalse), equalTo(efalse));
  }

  @Test
  public void testEntailment() {
    assertThat(etrue.entail(etrue), equalTo(Kleene.TRUE));
    assertThat(efalse.entail(etrue), equalTo(Kleene.TRUE));
    assertThat(etrue.entail(efalse), equalTo(Kleene.FALSE));
    assertThat(efalse.entail(efalse), equalTo(Kleene.TRUE));
    assertThat(eunknown.entail(etrue), equalTo(Kleene.FALSE));
    assertThat(etrue.entail(eunknown), equalTo(Kleene.TRUE));
    assertThat(eunknown.entail(eunknown), equalTo(Kleene.TRUE));
    assertThat(efalse.entail(eunknown), equalTo(Kleene.TRUE));
    assertThat(eunknown.entail(efalse), equalTo(Kleene.FALSE));
  }
}
