// Jean-Ferdy SUSINI, Frederic BOUSSINOT
// Copyright (c) 1997-2006. All rights reserved.
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

import inria.meije.rc.sugarcubes.implementation.*;
import inria.meije.rc.sugarcubes.*;

public class PauseUp extends InstructionImpl
{
  public boolean ended = false;

  public String actualToString(){
    if(ended){
      return "nothing";
    }
    return "pause up";
  }

  public String dumpToXMLRepresentation(String indent){
    return indent+"<pauseup state=\""+ended+"\"/>\n";
  }

  public Instruction copy(){
    return new PauseUp();
  }

  public Instruction prepareFor(Environment env){
    return copy();
  }

  public Instruction residual(){
    if(ended){
      return Nothing.NOTHING;
    }
    return new PauseUp();
  }

  public void notifyFreeze(Environment env){
    ended = false;
  }

  public void notifyTermination(Environment env){
    ended = false;
  }

  public byte activate(Environment e){
    SpaceEnvironment env = (SpaceEnvironment) e;
    env.pausedUp = true;
    if(ended){
      ended = false;
      return TERM;
    }
    ended = true;
    return STOP;
  }
}
