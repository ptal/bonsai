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

package bonsai.runtime.sugarcubes;

import java.util.function.*;
import bonsai.runtime.core.*;
import inria.meije.rc.sugarcubes.*;
import inria.meije.rc.sugarcubes.implementation.*;

public class SuspendWhen extends UnaryInstruction implements Precursor
{
  Config config;
  boolean confEvaluated;

  public SuspendWhen(Config config, Program body)
  {
    super(body);
    this.config = config;
    this.confEvaluated = false;
  }

  private SuspendWhen(SuspendWhen suspend) {
    super(suspend.body.copy());
    this.config = suspend.config.copy();
    this.confEvaluated = suspend.confEvaluated;
  }

  public String actualToString() {
    return "suspend when " + config + " { " + body + " }";
  }

  public SuspendWhen copy() {
    return new SuspendWhen(this);
  }

  public SuspendWhen prepareFor(Environment env) {
    SuspendWhen copy = new SuspendWhen(config.prepareFor(env), body.prepareFor(env));
    copy.body.setParent(copy);
    copy.config.setPrecursor(copy, Config.NEED_BOTH);
    return copy;
  }

  public byte activate(Environment e) {
    SpaceEnvironment env = (SpaceEnvironment) e;
    try{
      byte status = config.evaluate(env);
      switch (status) {
        case Config.UNKNOWN:
        case Config.UNKNOWN_UNTIL_EOI:
          return WEOI;
        case Config.SATISFIED:
          return STOP;
      }
      confEvaluated = true;
    }
    catch(Throwable t){
      t.printStackTrace();
      return EXCP;
    }
    byte res = body.activate(env);
    if((TERM == res) || (EXCP == res)){
      finish();
    }
    return res;
  }

  public byte activateOnEOI(Environment env){
    byte status = activate(env);
    confEvaluated = false;
    return status;
  }

  protected void finish(){
    config.reset();
    confEvaluated = false;
  }

  public void zap(Instruction from){
      parent.zap(this);
  }
  public void zapFromHere(Environment env){
      parent.zap(this);
  }
}

