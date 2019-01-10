// Copyright 2018 Pierre Talbot (IRCAM)

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
import bonsai.runtime.synchronous.expressions.*;
import bonsai.runtime.synchronous.interfaces.*;
import bonsai.runtime.synchronous.env.*;

public abstract class VarDecl extends ASTNode implements Statement
{
  protected String uid;
  protected Expression initValue;
  protected Statement body;

  protected StmtResult res;
  protected ExprResult exprResult;

  public VarDecl(String uid, Expression initValue, Statement body) {
    this.uid = uid;
    this.initValue = initValue;
    this.body = body;
    init();
  }

  private void init() {
    this.res = new StmtResult(CompletionCode.WAIT);
    this.exprResult = new ExprResult();
  }

  public HashSet<String> activeQueues(int layersRemaining) {
    if (layersRemaining == 0) {
      return new HashSet();
    }
    else {
      return body.activeQueues(layersRemaining);
    }
  }

  public CompletionCode endOfInstant(int layersRemaining, Layer layer) {
    checkNonTerminatedEOI("Var decl", res.k);
    if (state2()) {
      return body.endOfInstant(layersRemaining, layer);
    }
    else {
      return res.k;
    }
  }

  // in the initializing expression.
  protected boolean state1() {
    return res.k != CompletionCode.TERMINATE && exprResult.isSuspended();
  }

  // in the body.
  protected boolean state2() {
    return res.k != CompletionCode.TERMINATE && !exprResult.isSuspended();
  }

  // terminated.
  protected boolean state3() {
    return res.k == CompletionCode.TERMINATE;
  }

  protected void terminate(Layer layer) {
    // System.out.println("VarDecl.terminate");
    layer.exitScope(uid);
    // Free the pointed value of the variable.
    exprResult = new ExprResult();
  }

  public void prepare() {
    init();
    body.prepare();
    body.setParent(this);
  }

  public boolean canTerminate() {
    return body.canTerminate();
  }

  public void abort(Layer layer) {
    if (!state3()) {
      if (state1()) { // if the initializing expression is not terminated.
        initValue.terminate(layer);
      }
      body.abort(layer); // if the body was not terminated.
      if (state2()) { // if the body is active and not terminated.
        terminate(layer);
      }
      res = new StmtResult(CompletionCode.TERMINATE);
    }
  }

  public void suspend(Layer layer) {
    if (!state3()) {
      if (state1()) {
        initValue.terminate(layer);
      }
      body.suspend(layer);
    }
  }

  public StmtResult execute(int layersRemaining, Layer layer) {
    if (layersRemaining == 0) {
      executeState1(layer);
      executeState2(layer);
      return res;
    }
    else {
      if (state2()) {
        return body.execute(layersRemaining, layer);
      }
      else {
        hasNoSubLayer("VarDecl.execute");
        return null;
      }
    }
  }

  private void executeState1(Layer layer) {
    if (state1()) {
      // System.out.println("VarDecl.executeState1");
      exprResult = initValue.execute(layer);
      if (!exprResult.isSuspended()) {
        initValue.terminate(layer);
        layer.enterScope(uid, exprResult.unwrap(), (Object o) -> {});
      }
    }
  }

  private void executeState2(Layer layer) {
    if (state2()) {
      // System.out.println("VarDecl.executeState2");
      res = body.execute(0, layer);
      if (state3()) {
        terminate(layer);
      }
    }
  }

  public CanWriteOnResult canWriteOn(int layersRemaining, Layer layer, String uid, boolean inSurface) {
    if (layersRemaining == 0) {
      if (state1()) {
        if (initValue.canWriteOn(uid)) {
          return new CanWriteOnResult(canTerminate(), true);
        }
      }
      if (state1() || state2()) {
        return body.canWriteOn(layersRemaining, layer, uid, inSurface);
      }
      return new CanWriteOnResult(true, false);
    }
    else {
      return body.canWriteOn(layersRemaining, layer, uid, inSurface);
    }
  }

  public int countLayers() { return body.countLayers(); }
}
