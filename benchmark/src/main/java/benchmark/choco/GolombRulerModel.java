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

import org.chocosolver.solver.search.measure.MeasuresRecorder;
import org.chocosolver.solver.Model;
import org.chocosolver.solver.variables.IntVar;

import static org.chocosolver.solver.ResolutionPolicy.MINIMIZE;
import static org.chocosolver.solver.search.strategy.Search.inputOrderLBSearch;

/**
 * CSPLib prob006:<br/>
 * A Golomb ruler may be defined as a set of m integers 0 = a_1 < a_2 < ... < a_m such that
 * the m(m-1)/2 differences a_j - a_i, 1 <= i < j <= m are distinct.
 * Such a ruler is said to contain m marks and is of length a_m.
 * <br/>
 * The objective is to find optimal (minimum length) or near optimal rulers.
 * <br/>
 *
 * @author Charles Prud'homme
 * @since 31/03/11
 */
public class GolombRulerModel
{
  IntVar[] ticks;
  IntVar[] diffs;
  IntVar[][] m_diffs;
  int m;

  protected Model model;

  public GolombRulerModel() {
    this.m = Config.current.n;
    this.buildModel();
    this.configureSearch();
    model.setObjective(false, getObjective());
  }

  public IntVar getObjective() {
    return (IntVar) model.getVars()[m - 1];
  }

  public void solve() {
    while(model.getSolver().solve()) {
      Config.current.time = model.getSolver().getMeasures().getTimeCountInNanoSeconds();
      if (Config.current.hasTimedOut()) {
        throw new TimeLimitException();
      }
    }
    MeasuresRecorder m = model.getSolver().getMeasures();
    Config.current.nodes = m.getBackTrackCount();
    Config.current.solutions = m.getSolutionCount();
    Config.current.fails = m.getFailCount();
    Config.current.time = m.getTimeCountInNanoSeconds();
    Config.current.obj = (Integer) m.getBestSolutionValue();
  }

  public void buildModel() {
    model = new Model("GolombRuler");
    ticks = model.intVarArray("a", m, 0, (m < 31) ? (1 << (m + 1)) - 1 : 9999, true);
    model.arithm(ticks[0], "=", 0).post();
    for (int i = 0; i < m - 1; i++) {
      model.arithm(ticks[i + 1], ">", ticks[i]).post();
    }
    diffs = model.intVarArray("d", (m * m - m) / 2, 0, (m < 31) ? (1 << (m + 1)) - 1 : 9999, true);
    m_diffs = new IntVar[m][m];
    for (int k = 0, i = 0; i < m - 1; i++) {
      for (int j = i + 1; j < m; j++, k++) {
        // d[k] is m[j]-m[i] and must be at least sum of first j-i integers
        // <cpru 04/03/12> it is worth adding a constraint instead of a view
        model.scalar(new IntVar[]{ticks[j], ticks[i]}, new int[]{1, -1}, "=", diffs[k]).post();
        model.arithm(diffs[k], ">=", (j - i) * (j - i + 1) / 2).post();
        model.arithm(diffs[k], "-", ticks[m - 1], "<=", -((m - 1 - j + i) * (m - j + i)) / 2).post();
        model.arithm(diffs[k], "<=", ticks[m - 1], "-", ((m - 1 - j + i) * (m - j + i)) / 2).post();
        m_diffs[i][j] = diffs[k];
      }
    }
    model.allDifferent(diffs, "BC").post();
    // break symetries
    if (m > 2) {
      model.arithm(diffs[0], "<", diffs[diffs.length - 1]).post();
    }
  }

  public void configureSearch() {
    model.getSolver().setSearch(inputOrderLBSearch(ticks));
    model.getSolver().limitTime(Config.timeout + "s");
  }
}