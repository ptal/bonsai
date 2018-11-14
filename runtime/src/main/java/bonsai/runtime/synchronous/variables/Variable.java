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

import java.util.*;
import java.util.function.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.env.*;

public class Variable
{
  private String uid;
  private RWCounter rw;
  private Object value;
  private ArrayList<Consumer<Object>> refUpdaters;

  public Variable(String uid)
  {
    this.uid = uid;
    this.rw = new RWCounter(0,0,0);
    this.value = null;
    this.refUpdaters = new ArrayList();
  }

  public Object value() {
    return value;
  }

  // This method must be called each time `value` is allocated to a new reference.
  public void updateRefValue(Object value) {
    this.value = value;
    for (Consumer<Object> updater : refUpdaters) {
      updater.accept(this.value);
    }
  }

  public String uid() {
    return uid;
  }

  // We enter the scope of the current variable, or the scope of a `ref` field referencing this variable.
  public void enterScope(Object refValue, Consumer<Object> refUpdater) {
    if (this.value == null) {
      if (!refUpdaters.isEmpty()) {
        throw new RuntimeException("[BUG] The current value in `Variable` is `null` but some refUpdater are registered " +
          "(uid: " + uid + ").");
      }
      this.value = refValue;
    }
    else {
      if (this.value != refValue) {
        throw new BonsaiInterfaceException("A reference field (annotated with `ref`) " +
          "was initialized in the Java constructor with a different value (or `null`) than the one passed to the constructor.\n" +
          "UID of the field: `" + uid + "`.\n" +
          "Solution: Initialize the `ref` field in the constructor with the same object than the source object.");
      }
    }
    refUpdaters.add(refUpdater);
  }

  public void exitScope() {
    // The `refUpdaters` list can be empty if the variable did not enter its scope (it was only `Space.register`).
    if (!refUpdaters.isEmpty()) {
      refUpdaters.remove(refUpdaters.size() - 1);
    }
  }

  public boolean isInScope() {
    return refUpdaters.size() > 0;
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

  public String toString() {
    return "UID: " + uid + " " + rw + ": " + value;
  }
}
