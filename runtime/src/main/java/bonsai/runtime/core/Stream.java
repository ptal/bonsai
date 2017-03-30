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

package bonsai.runtime.core;

import java.util.function.*;

/// A bounded stream implemented with a circular buffer where pushing an element when the history is full just erase the first one.
public class Stream implements Restorable
{
  // The reference to the object in the Java class that we must always keep as the current object.
  private Object ref;
  // This contains a stream of object "of the past".
  private Object[] stream;
  private int size;
  private int last;

  private String name;
  private boolean isTransient;

  public Stream(Object ref, String name, int streamCapacity, boolean isTransient)
  {
    this.ref = ref;
    stream = new Object[streamCapacity];
    this.name = name;
    this.isTransient = isTransient;
    this.size = 0;
    this.last = 0;
  }

  public int capacity() {
    return stream.length;
  }

  public boolean isUnary() {
    return capacity() == 0;
  }

  public void reset(Object value) {
    this.size = 0;
    this.last = 0;
    resetRef(value);
  }

  private Object copyRef() {
    return Cast.toCopy(name, ref).copy();
  }

  private void resetRef(Object value) {
    Cast.toResettable(name, ref).reset(value);
  }

  private void push(Object value) {
    if (!isUnary()) {
      last = (last + 1) % capacity();
      stream[last] = copyRef();
      if (size < capacity()) {
        size = size + 1;
      }
    }
    resetRef(value);
  }

  public void next(Supplier<Object> defaultValue) {
    if (isTransient) {
      push(defaultValue.get());
    }
    else {
      if (!isUnary()) {
        duplicateLast();
      }
    }
  }

  private void duplicateLast() {
    push(copyRef());
  }

  // Access the value of the stream at T_{c-t} where `c` is the current instant.
  public Object pre(int t) {
    if (t == 0) {
      return ref;
    }
    t = t - 1;
    checkNonEmptyStream("pre");
    checkCapacity(t);
    // If the value does not have such a long history yet, return its bottom.
    if (t >= size) {
      return Cast.toLattice(name, ref).bottom();
    }
    else {
      return stream[preIndex(t)];
    }
  }

  private int preIndex(int t) {
    int x = last - t;
    if (x < 0) {
      x = capacity() + x;
    }
    return x;
  }

  /// The labelling of a stream works as follows:
  ///  (1) If the stream is unary then we use the `Restorable` interface of the variable.
  ///  (2) If the stream is n-ary then we just keep track of the references of the elements in the current stream.
  ///      The last element will be duplicated in `next()`.
  public Object label() {
    if (isUnary()) {
      return Cast.toRestorable(name, ref).label();
    }
    else {
      Label label = new Label(copyRef());
      int begin = preIndex(size-1);
      for (int i = 0; i < size; ++i) {
        Object element = stream[(begin+i)%capacity()];
        label.push(element);
      }
      return label;
    }
  }

  private class Label {
    private Object ref;
    private Object[] labels;
    private int size;
    public Label(Object ref) {
      this.ref = ref;
      labels = new Object[capacity()];
      size = 0;
    }
    public void push(Object label) {
      labels[size] = label;
      size = size + 1;
    }
  }

  public void restore(Object label) {
    if (isUnary()) {
      Cast.toRestorable(name, ref).restore(label);
    }
    else {
      Label lab = (Label) label;
      resetRef(lab.ref);
      stream = lab.labels;
      size = lab.size;
      last = size-1;
    }
  }

  private void checkNonEmptyStream(String method) {
    if (size == 0) {
      throw new RuntimeException(
        "[BUG] Try to access value of an empty stream for the variable `"
        + name + "` in the method `" + method + "`.");
    }
  }

  private void checkCapacity(int t) {
    if (t >= capacity()) {
      throw new RuntimeException(
        "[BUG] Try to access the past value of the variable `"
        + name + "` but the stream is bounded with an history of `"
        + capacity() + "` elements.");
    }
  }

  public String toString() {
    String s = "[ref=" + ref + ", ";
    for (int i = 0; i < capacity(); ++i) {
      s += stream[i] + ", ";
    }
    return name + " = " + s + "]";
  }
}
