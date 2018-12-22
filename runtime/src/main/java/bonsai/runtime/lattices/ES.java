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

// This lattice gives an order to Kleene values such that `False |= True |= Unknown`.

package bonsai.runtime.lattices;

import bonsai.runtime.core.*;
import static bonsai.runtime.core.Kleene.*;

public class ES implements Lattice {
  private Kleene value;

  public ES() {
    value = Kleene.UNKNOWN;
  }

  public ES(Kleene k) {
    value = k;
  }

  public Kleene unwrap() {
    return value;
  }

  public ES bottom() {
    return new ES(Kleene.UNKNOWN);
  }

  public ES top() {
    return new ES(Kleene.FALSE);
  }

  static Kleene[][] table_join = {
  //   this |_| o  TRUE  FALSE  UNKNOWN
    /* TRUE */    {TRUE, FALSE, TRUE},
    /* FALSE */   {FALSE, FALSE, FALSE},
    /* UNKNOWN */ {TRUE, FALSE, UNKNOWN}
  };
  public ES join(Object o) {
    ES e = castToES("join", o);
    return new ES(table_join[value.ordinal()][e.value.ordinal()]);
  }

  public void join_in_place(Object o) {
    value = join(o).value;
  }

  static Kleene[][] table_meet = {
  //   this |⁻| o  TRUE  FALSE  UNKNOWN
    /* TRUE */    {TRUE, TRUE, UNKNOWN},
    /* FALSE */   {TRUE, FALSE, UNKNOWN},
    /* UNKNOWN */ {UNKNOWN, UNKNOWN, UNKNOWN}
  };
  public ES meet(Object o) {
    ES e = castToES("meet", o);
    return new ES(table_meet[value.ordinal()][e.value.ordinal()]);
  }

  public void meet_in_place(Object o) {
    value = meet(o).value;
  }

  static Kleene[][] table_entail = {
  //   this |= o   TRUE  FALSE  UNKNOWN
    /* TRUE */    {TRUE, FALSE, TRUE},
    /* FALSE */   {TRUE, TRUE, TRUE},
    /* UNKNOWN */ {FALSE, FALSE, TRUE}
  };
  public Kleene entails(Object o) {
    ES e = castToES("entails", o);
    return table_entail[value.ordinal()][e.value.ordinal()];
  }

  public boolean equals(Object o) {
    ES e = castToES("eq", o);
    return value == e.value;
  }

  private ES castToES(String from, Object o) {
    Cast.checkNull("argument", from, o);
    if (o instanceof Kleene) {
      o = new ES((Kleene) o);
    }
    if (!(o.getClass() == ES.class)) {
      throw new ClassCastException("Operation `" + from + "` between type `ES` and type `"
        + o.getClass().getCanonicalName() + "` is not supported.");
    }
    return (ES) o;
  }
}
