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
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class SpacetimeVar extends UnaryInstruction
{
  protected String name;
  protected boolean isModuleAttr;
  protected Spacetime spacetime;
  protected Stream stream;
  protected Function<SpaceEnvironment, Object> initValue;
  private boolean firstActivation;

  public SpacetimeVar(String name, boolean isModuleAttr, Spacetime spacetime,
    Boolean isTransient, int streamSize,
    Function<SpaceEnvironment, Object> initValue, Program body)
  {
    this(name, isModuleAttr, spacetime, initValue,
      new Stream(name, streamSize, isTransient), body);
  }

  protected SpacetimeVar(String name, boolean isModuleAttr, Spacetime spacetime,
    Function<SpaceEnvironment, Object> initValue, Stream stream, Program body)
  {
    super(body);
    if (stream.capacity() != 1 && spacetime == Spacetime.SingleTime) {
      throw new RuntimeException(
        "Single time variable must have a maximum size of stream of 1. This is a bug.");
    }
    this.name = name;
    this.isModuleAttr = isModuleAttr;
    this.spacetime = spacetime;
    this.stream = stream;
    this.initValue = initValue;
    this.firstActivation = true;
  }

  public String actualToString() {
    return name + " in " + spacetime + " = " + value(0) + ";\n" + body;
  }

  public SpacetimeVar copy() {
    return new SpacetimeVar(name, isModuleAttr, spacetime, initValue, stream, body.copy());
  }

  public SpacetimeVar prepareFor(Environment env) {
    SpacetimeVar copy = new SpacetimeVar(name, isModuleAttr, spacetime,
      initValue, stream, body.prepareFor(env));
    copy.body.setParent(copy);
    /// If this variable is a module attribute, we must initialise it now in case it is used by parallel process before being run, consider `run o || when o.attr`. Also, it is safe to initialise it now because module attribute should not use the environment.
    if (isModuleAttr) {
      firstActivation((SpaceEnvironment) env);
    }
    return copy;
  }

  public byte activate(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    firstActivation(env);
    byte res = body.activate(env);
    lastActivation(env, res);
    return res;
  }

  public void firstActivation(SpaceEnvironment env) {
    if (firstActivation) {
      firstActivation = false;
      initialiseCurrentValue(env);
      env.declareVar(name, this);
    }
  }

  public void initialiseCurrentValue(SpaceEnvironment env) {
    Object value = initValue.apply(env);
    stream.reset(value);
  }

  /// Check if the variable exit its scope.
  public void lastActivation(SpaceEnvironment env, byte res) {
    if (TERM == res || EXCP == res) {
      firstActivation = true;
    }
  }

  public Object value(int time) {
    return stream.pre(time);
  }

  public void save(Snapshot snapshot) {
    if (spacetime == Spacetime.WorldLine) {
      snapshot.saveWorldLineVar(name, stream);
    }
    else if (spacetime == Spacetime.SingleTime) {
      snapshot.saveSingleTimeVar(name, value(0));
    }
  }

  public void restore(SpaceEnvironment env, Snapshot snapshot) {
    // We only restore variable that are activated (in scope).
    if (!firstActivation) {
      if (spacetime == Spacetime.WorldLine) {
        snapshot.restoreWorldLineVar(name, stream);
      }
      stream.next(() -> initValue.apply(env));
    }
  }
}
