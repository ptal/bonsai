// Copyright 2019 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package benchmark;

import java.util.*;

public class Config
{
  public static long timeout;

  public String problemName;
  public int n;
  public long nodes;
  public long solutions;
  public long fails;
  public long time;
  public long obj;

  public Config(String problemName, int n) {
    this.problemName = problemName;
    this.n = n;
    this.nodes = 0;
    this.solutions = 0;
    this.fails = 0;
    this.time = 0;
    this.obj = -1;
  }

  public static Config current;
  public static void init(String problemName, int n) {
    current = new Config(problemName, n);
  }

  public String toCSV(boolean human) {
    String line = problemName + "," + n + "," + nodes + "," + solutions + "," + fails + ",";
    if (hasTimedOut()) {
      line += "timeout,na,na";
    }
    else {
      if (human) {
        line += time + "(" + nanoToMS(time) + "ms)(" + nanoToSec(time) + "s),";
      }
      else {
        line += nanoToMS(time) + ",";
      }
      line += ((nodes*1000000000) / time);
      if (human) { line += "n/s,"; }
      else { line += ","; }
      if (obj != -1) {
        line += obj;
      }
      else {
        line += "na";
      }
    }
    return line;
  }

  public boolean hasTimedOut() {
    return nanoToSec(time) >= timeout;
  }

  public static String headerCSV() {
    return "problems,size,nodes,solutions,fails,time(timeout=" + timeout + "s),nodes per seconds,obj";
  }

  public static long nanoToSec(long nanoSec) {
    return nanoToMS(nanoSec) / 1000;
  }

  public static long nanoToMS(long nanoSec) {
    return nanoSec / 1000000;
  }
}
