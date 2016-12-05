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
package chococubes.example;

import org.chocosolver.solver.Model;
import org.chocosolver.solver.variables.IntVar;

import static org.chocosolver.solver.search.strategy.Search.inputOrderLBSearch;

public class LatinSquareChoco {

    private int m = 60;

    IntVar[] vars;

    protected Model model;

    /**
     * @return the current model
     */
    public Model getModel() {
        return model;
    }

    public void buildModel() {
        model = new Model("Latin square");
        vars = new IntVar[m * m];
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < m; j++) {
                vars[i * m + j] = model.intVar("C" + i + "_" + j, 0, m - 1, false);
            }
        }
        // Constraints
        for (int i = 0; i < m; i++) {
            IntVar[] row = new IntVar[m];
            IntVar[] col = new IntVar[m];
            for (int x = 0; x < m; x++) {
                row[x] = vars[i * m + x];
                col[x] = vars[x * m + i];
            }
            model.allDifferent(col, "AC").post();
            model.allDifferent(row, "AC").post();
        }
    }

    public void configureSearch() {
        model.getSolver().setSearch(inputOrderLBSearch(vars));
    }

    public void solve() {
        buildModel();
        configureSearch();
        model.getSolver().solve();
        // while(model.getSolver().solve()) {
        // }
        System.out.println(model.getSolver().getMeasures().toString());
    }

    public static void main(String[] args) {
        new LatinSquareChoco().solve();
    }
}