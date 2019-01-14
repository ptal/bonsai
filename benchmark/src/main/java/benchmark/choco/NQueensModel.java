/**
 * Copyright (c) 2016, chocoteam
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * * Redistributions of source code must retain the above copyright notice, this
 *   list of conditions and the following disclaimer.
 *
 * * Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 *
 * * Neither the name of samples nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
package benchmark.choco;

import benchmark.*;

import org.chocosolver.solver.Model;
import org.chocosolver.solver.variables.IntVar;
import org.chocosolver.solver.search.measure.MeasuresRecorder;

import static org.chocosolver.solver.search.strategy.Search.minDomLBSearch;

/**
 * <br/>
 *
 * @author Charles Prud'homme
 * @since 31/03/11
 */
public class NQueensModel {
  int n;
  IntVar[] vars;

  protected Model model;

  public NQueensModel() {
    this.n = Config.current.n;
    this.buildModel();
    this.configureSearch();
  }

  public void solve() {
    while(model.getSolver().solve()) {
      Config.current.time = model.getSolver().getMeasures().getTimeCountInNanoSeconds();
      if (Config.current.hasTimedOut()) {
        throw new TimeLimitException();
      }
    }
    MeasuresRecorder m = model.getSolver().getMeasures();
    Config.current.solutions = m.getSolutionCount();
    Config.current.fails = m.getFailCount();
    Config.current.nodes = Config.current.fails + m.getNodeCount();
    Config.current.time = m.getTimeCountInNanoSeconds();
  }

  public void buildModel() {
    model = new Model("NQueen");
    vars = new IntVar[n];
    IntVar[] diag1 = new IntVar[n];
    IntVar[] diag2 = new IntVar[n];
    for (int i = 0; i < n; i++) {
      vars[i] = model.intVar("Q_" + i, 1, n, false);
      diag1[i] = model.intOffsetView(vars[i], i);
      diag2[i] = model.intOffsetView(vars[i], -i);
    }
    model.allDifferent(vars, "BC").post();
    model.allDifferent(diag1, "BC").post();
    model.allDifferent(diag2, "BC").post();
  }

  public void configureSearch() {
    model.getSolver().setSearch(minDomLBSearch(vars));
    model.getSolver().limitTime(Config.timeout + "s");
  }
}
