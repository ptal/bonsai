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

/// `SpaceMachine` is the class in charge of executing a spacetime program.
/// The user control the execution of the program by calling the methods:
///   * `macroStep()` to execute the program until it either `pause up`, `stop`, terminates or if the queue is empty.
///   * `pop()` to prepare the program for the next instant.
/// One must alternate the execution of `macroStep` and `pop` because the variables are only:
///   * Readable after executing `macroStep()`: they represent the value computed until the end of the current instant.
///   * Writeable after executing `pop()`: we prepared the variables for the next instant, and more information can be injected (but not read because the current instant has not been executed yet).
/// For example, assuming we have a class `NQueens` computing the n-queens problem:
/*
  int N = 8;
  NQueens nqueens = new NQueens(N);
  int nodes = 0;
  SpaceMachine<StackLR> machine = SpaceMachine.create(nqueens, new StackLR());
  while (machine.macroStep()) {
    // Print the intermediate solution.
    System.out.println(nqueens);
    machine.pop();
    // Add some random constraints, just to show that constraint can be dynamically added during the exploration.
    // The constraint forces the `nodes % N` queen to be less or equal to `nodes % N`.
    nqueens.constraints().join(nqueens.queen(nodes % N).le(nodes % N));
    nodes++;
  }
*/

package bonsai.runtime.synchronous;

import java.util.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.interfaces.*;

public class SpaceMachine<Queue extends Queueing<Future>>
{
  private Program body;
  private Environment env;
  private boolean firstInstant;
  private boolean readyToStartInstant;

  private boolean debug;
  private int numReactions;

  public static <Q extends Queueing<Future>> SpaceMachine<Q> create(Program body, Q queue) {
    return new SpaceMachine(body, queue);
  }

  public static <Q extends Queueing<Future>> SpaceMachine<Q> createDebug(Program body, Q queue) {
    SpaceMachine machine = SpaceMachine.create(body, queue);
    machine.debug = true;
    return machine;
  }

  public SpaceMachine(Program body, Queue queue) {
    this.env = new Environment(queue);
    debug = false;
    numReactions = 0;
    readyToStartInstant = false;
    firstInstant = true;
  }

  // Returns `true` if the program is paused (through a `stop` or `pause up` statement).
  // If the program is terminated or the queue empty, it returns `false`.
  public boolean macroStep() {
    log_debug_info("Start of execution");
    firstPop();
    CompletionCode code = CompletionCode.PAUSE;
    while(code == CompletionCode.PAUSE && readyToStartInstant) {
      code = react();
    }
    log_debug_info("End of execution (code: " + code + ")");
    return code != CompletionCode.TERMINATE && !env.isEmpty();
  }

  private void firstPop() {
    if(!firstInstant) {
      pop();
    }
    else {
      readyToStartInstant = true;
      firstInstant = false;
    }
  }

  private CompletionCode react() {
    CompletionCode code = microStep();
    env.pushBranches();
    readyToStartInstant = false;
    switch(code) {
      case MICRO_STEP:
        throw new RuntimeException(
          "Completion code after a micro step cannot equal to `MICRO_STEP`.");
      case PAUSE:
        pop();
        break;
      default: break;
    }
    return code;
  }

  // This method is used for explicitly instantiating a future.
  // Returns `true` if the machine is ready for a new instant, and `false` if no more future is available.
  // If `pop()` has already been called during the current instant, a `RuntimeException` is thrown.
  public boolean pop() {
    if(!readyToStartInstant) {
      Optional<Future> future = env.pop();
      if (future.isPresent()) {
        branchMicroStep(future.get());
        readyToStartInstant = true;
      }
      return readyToStartInstant;
    }
    throw new RuntimeException(
      "The method `SpaceMachine.pop()` has already in the current instant.\n" +
      "Please call this method only one time per instant.");
  }

  private void log_debug_info(String status) {
    if(debug) {
      System.out.println("Reaction no " + numReactions);
      System.out.println("  Execution status: " + status);
      System.out.println("  Queue size: " + env.queueSize());
    }
  }

  private CompletionCode microStep() {
    numReactions++;
    log_debug_info("Start of the instant");
    CompletionCode result = CompletionCode.MICRO_STEP;
    while(result == CompletionCode.MICRO_STEP) {
      result = body.execute(env);
      switch(result) {
        case MICRO_STEP:
          // body.countReadWrite(env);
          break;
        default: break;
      }
    }
    return result;
  }

  // We execute a branch process with the `single_time` variables associated to its `future`.
  // The branch process must terminate immediately, and must not create branches.
  private void branchMicroStep(Future future) {
    // future.swapVarsST(env);
    // Program branch = future.branch();
    // CompletionCode code = CompletionCode.MICRO_STEP;
    // while(code == CompletionCode.MICRO_STEP) {
    //   CompletionCode result = branch.execute(env);
    //   checkEmptyBranches(result);
    //   code = result.code();
    //   switch (code) {
    //     case MICRO_STEP:
    //       branch.countReadWrite(env);
    //       break;
    //     default: break;
    //   }
    // }
    // future.swapVarsST(env);
    // if (code != CompletionCode.TERMINATE) {
    //   throw new RuntimeException(
    //     "A branch process must terminate immediately (code obtained: " + code + ")");
    // }
  }

  // private void checkEmptyBranches(Result result) {
  //   if (!result.branches().isEmpty()) {
  //     throw new RuntimeException(
  //       "A branch process must not create new branches (code obtained: " + result.code() + ")");
  //   }
  // }
}
