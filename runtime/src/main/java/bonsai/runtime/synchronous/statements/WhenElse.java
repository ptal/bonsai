// Copyright 2018 Pierre Talbot

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package bonsai.runtime.synchronous.statements;

import java.util.*;
import java.util.function.*;
import bonsai.runtime.core.*;
import bonsai.runtime.synchronous.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;
import bonsai.runtime.synchronous.expressions.*;
import bonsai.runtime.lattices.*;

public class WhenElse extends ASTNode implements Statement
{
  private final Entailment cond;
  private final Statement then;
  private final Statement els;

  private StmtResult res;
  private ExprResult condResult;

  public WhenElse(Entailment cond, Statement then, Statement els) {
    super();
    this.cond = cond;
    this.then = then;
    this.els = els;
    init();
  }

  public WhenElse copy() {
    return new WhenElse(cond.copy(), then.copy(), els.copy());
  }

  private void init() {
    res = new StmtResult(CompletionCode.WAIT);
    condResult = new ExprResult();
  }

  public void prepare() {
    then.prepare();
    els.prepare();
    then.setParent(this);
    els.setParent(this);
    init();
  }

  private Kleene condition() {
    if (condResult.isSuspended()) {
      return Kleene.UNKNOWN;
    }
    else {
      Object r = condResult.unwrap();
      if (r instanceof ES) {
        return ((ES) r).unwrap();
      }
      else if (r instanceof Kleene) {
        return (Kleene) r;
      }
      else {
        throw new RuntimeException("A condition in a `when` statement has type `" + r.getClass().getName() + "`\n"
          + "Object value: " + r);
      }
    }
  }

  // in the initializing expression.
  private boolean state1() {
    return res.k != CompletionCode.TERMINATE && condition() == Kleene.UNKNOWN;
  }

  // in the then branch.
  private boolean state2a() {
    return res.k != CompletionCode.TERMINATE && condition() == Kleene.TRUE;
  }

  // in the else branch.
  private boolean state2b() {
    return res.k != CompletionCode.TERMINATE && condition() == Kleene.FALSE;
  }

  // terminated.
  private boolean state3() {
    return res.k == CompletionCode.TERMINATE;
  }

  public void canBranchOrDefault(boolean extraCond, Consumer<Entailment> expr, Consumer<Statement> branch) {
    boolean canCond = (extraCond && state1());
    if(canCond) {
      expr.accept(cond);
    }
    if(canCond || state2a()) {
      branch.accept(then);
    }
    if(canCond || state2b()) {
      branch.accept(els);
    }
  }

  public void canInstant(int layersRemaining, Layer layer) {
    canBranchOrDefault(layersRemaining == 0,
      e -> e.canInstant(layer),
      s -> s.canInstant(layersRemaining, layer));
  }

  public <T> T branchOrDefault(Function<Statement, T> branch, Supplier<T> def) {
    if (state2a()) {
      return branch.apply(then);
    }
    else if (state2b()) {
      return branch.apply(els);
    }
    else {
      return def.get();
    }
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    return branchOrDefault(
      s -> s.activeQueues(layersRemaining),
      () -> new HashSet());
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    checkNonTerminatedEOI("when", res.k);
    checkExpressionStateEOI("when", state1());
    return branchOrDefault(
      s -> { res.k = s.endOfInstant(layersRemaining, layer); return res.k; },
      () -> { throw new RuntimeException("[BUG] Unreachable state in WhenElse.endOfInstant");});
  }

  public boolean canTerminate() {
    if (state1()) {
      return then.canTerminate() || els.canTerminate();
    }
    return branchOrDefault(s -> s.canTerminate(), () -> true);
  }

  public void abort(Layer layer) {
    canBranchOrDefault(true,
      e -> e.terminate(layer),
      s -> s.abort(layer));
  }

  public void suspend(Layer layer) {
    canBranchOrDefault(true,
      e -> e.terminate(layer),
      s -> s.suspend(layer));
  }

  public StmtResult execute(int layersRemaining, Layer layer) {
    if (layersRemaining == 0) {
      executeState1(layer);
      return branchOrDefault(
        s -> { res = s.execute(layersRemaining, layer); return res; },
        () -> res
      );
    }
    else {
      checkExpressionStateEOI("when", state1());
      return branchOrDefault(
        s -> s.execute(layersRemaining, layer),
        () -> { throw new RuntimeException("WhenElse.execute: unreachable path.");});
    }
  }

  private void executeState1(Layer layer) {
    if (state1()) {
      condResult = cond.execute(layer);
      if (!condResult.isSuspended()) {
        cond.terminate(layer);
        if (state2a()) {
          els.abort(layer);
        }
        else if (state2b()) {
          then.abort(layer);
        }
      }
    }
  }

  public CanWriteOnResult canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    // System.out.println("WhenElse.canWriteOn: " + uid);
    if (layersRemaining == 0 && state1()) {
      if(inSurface) {
        Kleene k = cond.execute(layer, uid);
        // System.out.println("WhenElse.canWriteOn: " + uid);
        if (k != null) {
          switch (k) {
            case TRUE:
              // System.out.println("WhenElse.canWriteOn.TRUE: " + uid);
              layer.subscribeUnblocked(uid, cond, k);
              return then.canWriteOn(layersRemaining, layer, uid, false);
            case FALSE:
              // System.out.println("WhenElse.canWriteOn.FALSE: " + uid);
              layer.subscribeUnblocked(uid, cond, k);
              return els.canWriteOn(layersRemaining, layer, uid, false);
            case UNKNOWN: throw new RuntimeException(
              "[BUG] Entailment.execute(layer,uid) returned an unknown result.");
          }
        }
      }
      return then.canWriteOn(layersRemaining, layer, uid, inSurface)
       .join(els.canWriteOn(layersRemaining, layer, uid, inSurface));
    }
    else {
      return branchOrDefault(
        s -> s.canWriteOn(layersRemaining, layer, uid, inSurface),
        () -> new CanWriteOnResult(true, false));
    }
  }

  public int countLayers() {
    return Math.max(then.countLayers(), els.countLayers());
  }
}
