// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.runtime.sugarcubes;

import java.util.function.*;
import bonsai.runtime.core.*;

public class SpacetimeVar extends Variable
{
  private Spacetime spacetime;
  private Stream stream;
  private Function<SpaceEnvironment, Object> initValue;

  public SpacetimeVar(Object ref, String name, String uid, Spacetime spacetime,
    Boolean isTransient, int streamSize,
    Function<SpaceEnvironment, Object> initValue)
  {
    this(name, uid, spacetime, initValue,
      new Stream(ref, name, streamSize, isTransient));
  }

  private SpacetimeVar(String name, String uid, Spacetime spacetime,
    Function<SpaceEnvironment, Object> initValue, Stream stream)
  {
    super(name, uid);
    if (stream.capacity() != 0 && spacetime == Spacetime.SingleTime) {
      throw new RuntimeException(
        "Single time variable cannot have a stream of past values. This is a bug.");
    }
    this.spacetime = spacetime;
    this.stream = stream;
    this.initValue = initValue;
  }

  public void reset(SpaceEnvironment env) {
    Object value = initValue.apply(env);
    stream.reset(value);
  }

  public Object value(int time) {
    return stream.pre(time);
  }

  public void save(Snapshot snapshot) {
    if (spacetime == Spacetime.WorldLine) {
      snapshot.saveWorldLineVar(uid(), stream);
    }
    else if (spacetime == Spacetime.SingleTime) {
      snapshot.saveSingleTimeVar(uid(), value(0));
    }
  }

  public void restore(SpaceEnvironment env, Snapshot snapshot) {
    if (spacetime == Spacetime.WorldLine) {
      snapshot.restoreWorldLineVar(uid(), stream);
    }
    stream.next(() -> initValue.apply(env));
  }

  public Spacetime spacetime() {
    return spacetime;
  }
}
