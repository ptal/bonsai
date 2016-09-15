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

#![allow(dead_code)]
#![allow(non_snake_case)]
grammar! bonsai2 {
  // #![show_api]

  program = spacing item+

  item
    = stmt_list
    / FN identifier LPAREN list_ident RPAREN LBRACE stmt_list RBRACE

  list_ident = identifier (COMMA identifier)* / ""

  stmt_list = stmt+

  stmt
    = PAR BARBAR? stmt_list (BARBAR stmt_list)* END
    / SPACE BARBAR? stmt_list (BARBAR stmt_list)* END
    / LET TRANSIENT? identifier (COLON java_ty)? IN spacetime EQ expr SEMI_COLON
    / LET identifier EQ identifier LEFT_ARROW expr SEMI_COLON
    / WHEN entailment LBRACE stmt_list RBRACE
    / PAUSE SEMI_COLON
    / TRAP identifier LBRACE stmt_list RBRACE
    / EXIT identifier SEMI_COLON
    / LOOP LBRACE stmt_list RBRACE
    / identifier LPAREN list_ident RPAREN SEMI_COLON
    / var LEFT_ARROW expr SEMI_COLON

  expr
    = java_expr
    / stream_var

  java_ty
    = identifier

  // We consider single identifier to be part of bonsai language.
  java_expr
    = NEW identifier LPAREN list_expr RPAREN
    / identifier (DOT identifier (LPAREN list_expr RPAREN)?)+
    / number
    / string_literal

  list_expr
    = expr (COMMA expr)*
    / ""

  entailment = stream_var ENTAILMENT expr

  stream_var = PRE* identifier (LBRACKET list_stream_var RBRACKET)?
  list_stream_var = stream_var (COMMA stream_var)*

  var = identifier (LBRACKET list_var RBRACKET)?
  list_var = var (COMMA var)*

  spacetime = WORLD_LINE / SINGLE_TIME / SINGLE_SPACE / identifier

  identifier = !digit !(keyword !ident_char) ident_char+ spacing -> (^) //> to_string
  ident_char = ["a-zA-Z0-9_"]

  number = digits -> (^) //> make_number
  digits = digit+ (UNDERSCORE* digit)* //> concat
  digit = ["0-9"]

  // TODO: proper escape mechanism
  string_literal = "\"" (!"\"" .)* "\"" -> (^)

  keyword
    = "let" / "fn" / "par" / "space" / "end" / "transient" / "pre" / "when"
    / "loop" / "pause" / "trap" / "exit" / "in" / "world_line"
    / "single_time" / "single_space" / "bot" / "top" / java_kw
  kw_tail = !ident_char spacing

  LET = "let" kw_tail
  FN = "fn" kw_tail
  PAR = "par" kw_tail
  SPACE = "space" kw_tail
  END = "end" kw_tail
  TRANSIENT = "transient" kw_tail //-> ()
  PRE = "pre" kw_tail
  WHEN = "when" kw_tail
  LOOP = "loop" kw_tail
  PAUSE = "pause" kw_tail
  TRAP = "trap" kw_tail
  EXIT = "exit" kw_tail
  IN = "in" kw_tail
  WORLD_LINE = "world_line" kw_tail
  SINGLE_TIME = "single_time" kw_tail
  SINGLE_SPACE = "single_space" kw_tail
  BOT = "bot" kw_tail
  TOP = "top" kw_tail

  UNDERSCORE = "_"
  DOTDOT = ".." spacing
  SEMI_COLON = ";" spacing
  COLON = ":" spacing
  EQ = "=" spacing
  COMMA = "," spacing
  DOT = "." spacing
  BARBAR = "||" spacing
  ENTAILMENT = "|=" spacing
  LEFT_ARROW = "<-" spacing
  BIND_OP = "=" spacing
  ADD_OP = "+" spacing
  SUB_OP = "-" spacing
  MUL_OP = "*" spacing
  LPAREN = "(" spacing
  RPAREN = ")" spacing
  LBRACKET = "[" spacing
  RBRACKET = "]" spacing
  LBRACE = "{" spacing
  RBRACE = "}" spacing

  spacing = [" \n\r\t"]* -> (^)

  // Java keyword
  java_kw = "new"
  NEW = "new" kw_tail

  // fn to_string(raw_text: Vec<char>) -> String {
  //   raw_text.into_iter().collect()
  // }
}


#[cfg(test)]
mod test
{
  use oak_runtime::*;
  use oak_runtime::ParseResult::*;
  use super::*;

  #[test]
  fn test_grammar()
  {
    let state = bonsai2::recognize_program("
      let variables in world_line = new VarStore();
      let constraints in world_line = new ConstraintStore(variables);
      let consistent in single_time = new Consistent();

      fn first_fail_middle() {
        let var in single_space = new FirstFail();
        let val in single_space = new IntDomainMin();
        loop {
          when consistent |= Consistent.Unknown {
            let x in variables = var.getVariable(variables.vars());
            let mid:int in single_time = val.selectValue(variables[x]);
            space
            || constraints[variables] <- new ArithCons(x, \">\", mid);
            || constraints[variables] <- new ArithCons(x, \"<=\", mid);
            end
          }
          pause;
        }
      }

      fn model() {
        let queen1 = variables <- new IntDomain(0,1);
        let queen2 = variables <- new IntDomain(0,1);

        constraints[variables] <- new ArithCons(queen1, \"!=\", queen2);
        let fake in single_time = new Fake(pre pre queens1[pre queens, queen2], new Object());
      }

      fn propagation() {
        loop {
          consistent <- Solver.propagate(constraints, variables);
          pause;
        }
      }

      fn one_solution() {
        loop {
          when consistent |= Consistent.True {
            exit FoundSolution;
          }
          pause;
        }
      }

      fn engine() {
        trap FoundSolution {
          par
          || fail_first_middle();
          || propagation();
          || one_solution();
          end
        }
      }

     par
     || model();
     || engine();
     end
     ".into_state());
    let result = state.into_result();
    println!("{:?}", result);
    match result {
      Success(_) => (),
      Partial(_, _)
    | Failure(_) => assert!(false)
    };
  }
}
