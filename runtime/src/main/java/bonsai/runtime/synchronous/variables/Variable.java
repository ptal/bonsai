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

package bonsai.runtime.synchronous.variables;

import bonsai.runtime.synchronous.env.*;

public abstract class Variable
{
  private String uid;
  private RWCounter rw;
  private boolean inScope;

  public Variable(String uid)
  {
    this.uid = uid;
    this.rw = new RWCounter(0,0,0);
    this.inScope = true;
  }

  public String uid() {
    return uid;
  }

  public void exitFromScope() {
    inScope = false;
  }

  public boolean isInScope() {
    return inScope;
  }

  public boolean isReadable() {
    return rw.isReadable();
  }

  public boolean isReadWritable() {
    return rw.isReadWritable();
  }

  public void joinRead(Layer env) {
    rw.read += 1;
  }

  public void joinReadWrite(Layer env) {
    rw.readwrite += 1;
  }

  public void joinWrite(Layer env) {
    rw.write += 1;
  }

  public void meetRead(Layer env) {
    rw.read -= 1;
  }

  public void meetReadWrite(Layer env) {
    rw.readwrite -= 1;
    anyEvent(env);
    if (rw.readwrite == 0 && rw.read != 0) {
      canReadEvent(env);
    }
  }

  public void meetWrite(Layer env) {
    rw.write -= 1;
    anyEvent(env);
    if (rw.write == 0) {
      if (rw.readwrite == 0) {
        canReadEvent(env);
      }
      else {
        canReadWriteEvent(env);
      }
    }
  }

  private void canReadEvent(Layer env) {
    env.schedule(new Event(uid(), Event.CAN_READ));
  }

  private void canReadWriteEvent(Layer env) {
    env.schedule(new Event(uid(), Event.CAN_READWRITE));
  }

  private void anyEvent(Layer env) {
    env.schedule(new Event(uid(), Event.ANY));
  }

  public abstract Object value();
}
