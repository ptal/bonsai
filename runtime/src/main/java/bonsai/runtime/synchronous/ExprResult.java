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

package bonsai.runtime.synchronous;

import java.util.*;

public class ExprResult {
  /* result is empty if the expression is suspended. */
  private Optional<Object> result;

  public ExprResult(Object result) {
    this.result = Optional.of(result);
  }

  public ExprResult() {
    this.result = Optional.empty();
  }

  public boolean isSuspended() {
    return !result.isPresent();
  }

  public Object unwrap() {
    checkResultNotEmpty();
    return result.get();
  }

  public void checkResultNotEmpty() {
    if(isSuspended()) {
      throw new NoSuchElementException(
        "Try to access the result of an expression but it was suspended, and did not produce any result yet.");
    }
  }
}
