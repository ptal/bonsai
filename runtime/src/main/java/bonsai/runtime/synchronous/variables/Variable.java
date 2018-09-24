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

import bonsai.runtime.synchronous.*;

// A variable is a tuple `(name, rw, refsCounter)` where `refsCounter` is the number of `space` processes that have captured this variable.
public abstract class Variable
{
  private String name;
  private Integer uid;
  private RWCounter rw;
  private int refsCounter;

  public Variable(String name)
  {
    this.name = name;
    this.uid = 0;
    this.rw = new RWCounter(0,0,0);
    this.refsCounter = 1;
  }

  public String name() {
    return name;
  }

  public Integer uid() {
    return uid;
  }

  public void assignUID(Integer uid) {
    this.uid = uid;
  }

  public int refs() {
    return refsCounter;
  }

  public void decreaseRefs() {
    refsCounter -= 1;
  }

  public boolean isReadable() {
    return rw.isReadable();
  }

  public boolean isReadWritable() {
    return rw.isReadWritable();
  }

  public void joinRead(Environment env) {
    rw.read += 1;
  }

  public void joinReadWrite(Environment env) {
    rw.readwrite += 1;
  }

  public void joinWrite(Environment env) {
    rw.write += 1;
  }

  public void meetRead(Environment env) {
    rw.read -= 1;
  }

  public void meetReadWrite(Environment env) {
    rw.readwrite -= 1;
    if (rw.readwrite == 0 && rw.read != 0) {
      Event event = Event.makeCanRead(this);
      env.schedule(event);
    }
  }

  public void meetWrite(Environment env) {
    rw.write -= 1;
    if (rw.write == 0) {
      Event event;
      if (rw.readwrite == 0) {
        event = Event.makeCanRead(this);
      }
      else {
        event = Event.makeCanReadWrite(this);
      }
      env.schedule(event);
    }
  }

  public abstract Object value();
}
