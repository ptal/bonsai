// Copyright 2017 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.runtime.synchronous.variables;

import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;

public class StreamVar extends Variable
{
  protected Stream stream;
  protected Function<Environment, Object> initValue;

  public StreamVar(Object ref, String name,
    int streamSize,
    Function<Environment, Object> initValue)
  {
    this(name, initValue,
      new Stream(ref, name, streamSize));
  }

  private StreamVar(String name,
    Function<Environment, Object> initValue, Stream stream)
  {
    super(name);
    this.stream = stream;
    this.initValue = initValue;
  }

  public void reset(Environment env) {
    Object value = initValue.apply(env);
    stream.reset(value);
  }

  public Object value() {
    return pre(0);
  }

  public Object pre(int time) {
    return stream.pre(time);
  }

  public Stream stream() {
    return stream;
  }
}
