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
  protected Function<SpaceEnvironment, Object> initValue;
  protected Object value;
  private boolean firstActivation;

  public SpacetimeVar(String name, boolean isModuleAttr, Spacetime spacetime,
    Function<SpaceEnvironment, Object> initValue, Program body)
  {
    super(body);
    this.name = name;
    this.isModuleAttr = isModuleAttr;
    this.spacetime = spacetime;
    this.initValue = initValue;
    this.firstActivation = true;
  }

  public String actualToString() {
    return name + " in " + spacetime + " = " + value + ";\n" + body;
  }

  public Instruction copy() {
    return new SpacetimeVar(name, isModuleAttr, spacetime, initValue, body.copy());
  }

  public Instruction prepareFor(Environment env) {
    SpacetimeVar copy = new SpacetimeVar(name, isModuleAttr, spacetime, initValue, body.prepareFor(env));
    copy.body.setParent(copy);
    /// If this variable is a module attribute, we must initialise it now in case it is used by parallel process before being run, consider `run o || when o.attr`. Also, it is safe to initialise it now because it should not use the environment.
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
      value = initValue.apply(env);
      env.declareVar(name, this);
    }
  }

  public void lastActivation(SpaceEnvironment env, byte res) {
    if (TERM == res || EXCP == res) {
      firstActivation = true;
    }
  }

  public Object value() {
    return value;
  }

  public void save(Snapshot snapshot) {
    if (spacetime == Spacetime.WorldLine) {
      snapshot.saveWorldLineVar(name, value);
    }
    else if (spacetime == Spacetime.SingleTime) {
      snapshot.saveSingleTimeVar(name, value);
    }
  }

  public void restore(SpaceEnvironment env, Snapshot snapshot) {
    if (spacetime == Spacetime.WorldLine) {
      snapshot.restoreWorldLineVar(name, value);
    }
    else if (spacetime == Spacetime.SingleTime) {
      if (!firstActivation) {
        value = initValue.apply(env);
      }
    }
  }
}
