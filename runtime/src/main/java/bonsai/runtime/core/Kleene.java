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

import static bonsai.runtime.core.Kleene.*;

public enum Kleene {
  // Caution: Do not reorder these values because truth table depends on this order.
  TRUE,
  FALSE,
  UNKNOWN;

  static Kleene[] table_not = {FALSE, TRUE, UNKNOWN};
  public static Kleene not(Kleene k) {
    return table_not[k.ordinal()];
  }

  static Kleene[][] table_and = {
    //               TRUE  FALSE  UNKNOWN
      /* TRUE */    {TRUE, FALSE, UNKNOWN},
      /* FALSE */   {FALSE, FALSE, FALSE},
      /* UNKNOWN */ {UNKNOWN, FALSE, UNKNOWN}
  };
  public static Kleene and(Kleene k, Kleene l) {
    return table_and[k.ordinal()][l.ordinal()];
  }

  static Kleene[][] table_or = {
    //               TRUE  FALSE  UNKNOWN
      /* TRUE */    {TRUE, TRUE, TRUE},
      /* FALSE */   {TRUE, FALSE, UNKNOWN},
      /* UNKNOWN */ {TRUE, UNKNOWN, UNKNOWN}
  };
  public static Kleene or(Kleene k, Kleene l) {

    return table_or[k.ordinal()][l.ordinal()];
  }

  public static Kleene fromBool(boolean b) {
    return b ? Kleene.TRUE : Kleene.FALSE;
  }

  public String toString() {
    switch(this) {
      case UNKNOWN: return "unknown";
      case TRUE: return "true";
      default: return "false";
    }
  }
}
