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
grammar! bonsai {
  // #![show_api]
  use std::str::FromStr;
  use ast::*;

  program = spacing item+

  item
    = stmt > make_stmt_item
    / FN identifier LPAREN list_ident RPAREN block > make_function_item

  fn make_stmt_item(stmt: Stmt) -> Item {
    Item::Statement(stmt)
  }

  fn make_function_item(name: String, params: Vec<String>, body: Block) -> Item {
    Item::Fn(Function {
      name: name,
      params: params,
      body: body
    })
  }

  list_ident
    = identifier (COMMA identifier)* > make_list_ident
    / "" > empty_ident_list

  fn make_list_ident(first: String, rest: Vec<String>) -> Vec<String> {
    extend_front(first, rest)
  }

  fn empty_ident_list() -> Vec<String> {
    vec![]
  }

  stmt_list = stmt+

  block = LBRACE stmt_list RBRACE

  stmt
    = PAR BARBAR? stmt_list (BARBAR stmt_list)* END > make_par
    / SPACE BARBAR? stmt_list (BARBAR stmt_list)* END > make_space
    / LET TRANSIENT? identifier (COLON java_ty)? IN spacetime EQ expr SEMI_COLON > make_let
    / LET identifier (COLON java_ty)? EQ identifier LEFT_ARROW expr SEMI_COLON > make_let_in_store
    / WHEN entailment block > make_when
    / PAUSE SEMI_COLON > make_pause
    / TRAP identifier block > make_trap
    / EXIT identifier SEMI_COLON > make_exit
    / LOOP block > make_loop
    / identifier LPAREN list_ident RPAREN SEMI_COLON > make_fn_call
    / var LEFT_ARROW expr SEMI_COLON > make_tell

  fn make_par(first: Block, rest: Vec<Block>) -> Stmt {
    Stmt::Par(extend_front(first, rest))
  }

  fn make_space(first: Block, rest: Vec<Block>) -> Stmt {
    Stmt::Space(extend_front(first, rest))
  }

  fn make_let(transient: Option<()>, var_name: String,
    var_ty: Option<JavaTy>, spacetime: Spacetime, expr: Expr) -> Stmt
  {
    let decl = LetDecl {
      transient: transient.is_some(),
      var: var_name,
      var_ty: var_ty,
      spacetime: spacetime,
      expr: expr
    };
    Stmt::Let(decl)
  }

  fn make_let_in_store(location: String, loc_ty: Option<JavaTy>, store: String, expr: Expr) -> Stmt {
    let decl = LetInStoreDecl {
      location: location,
      loc_ty: loc_ty,
      store: store,
      expr: expr
    };
    Stmt::LetInStore(decl)
  }

  fn make_when(entailment: EntailmentRel, body: Block) -> Stmt {
    Stmt::When(entailment, body)
  }

  fn make_pause() -> Stmt {
    Stmt::Pause
  }

  fn make_trap(name: String, block: Block) -> Stmt {
    Stmt::Trap(name, block)
  }

  fn make_exit(name: String) -> Stmt {
    Stmt::Exit(name)
  }

  fn make_loop(block: Block) -> Stmt {
    Stmt::Loop(block)
  }

  fn make_fn_call(fn_name: String, args: Vec<String>) -> Stmt {
    Stmt::FnCall(fn_name, args)
  }

  fn make_tell(var: Var, expr: Expr) -> Stmt {
    Stmt::Tell(var, expr)
  }

  expr
    = java_expr
    / stream_var > make_stream_var_expr

  fn make_stream_var_expr(var: StreamVar) -> Expr { Expr::Variable(var) }

  java_ty
    = identifier java_generic_list > make_java_ty

  fn make_java_ty(name: String, generics: Vec<JavaTy>) -> JavaTy {
    JavaTy {
      name: name,
      generics: generics
    }
  }

  java_generic_list
    = LT java_ty (COMMA java_ty)* GT? > make_generic_list
    / "" > empty_generic_list

  fn make_generic_list(first: JavaTy, rest: Vec<JavaTy>) -> Vec<JavaTy> {
    extend_front(first, rest)
  }

  fn empty_generic_list() -> Vec<JavaTy> {
    vec![]
  }

  // We consider single identifier to be part of bonsai language.
  java_expr
    = NEW java_ty LPAREN list_expr RPAREN > java_new
    / identifier java_call+ > java_object_calls
    / number > make_number_expr
    / string_literal > make_string_literal

  fn java_new(class_ty: JavaTy, args: Vec<Expr>) -> Expr {
    Expr::JavaNew(class_ty, args)
  }

  fn java_object_calls(object: String, calls: Vec<JavaCall>) -> Expr {
    Expr::JavaObjectCall(object, calls)
  }

  fn make_number_expr(n: u64) -> Expr { Expr::Number(n) }
  fn make_string_literal(lit: String) -> Expr { Expr::StringLiteral(lit) }

  java_call = DOT identifier (LPAREN list_expr RPAREN)? > make_java_call

  fn make_java_call(property: String, args: Option<Vec<Expr>>) -> JavaCall {
    JavaCall {
      property: property,
      args: args.unwrap_or(vec![])
    }
  }

  list_expr
    = expr (COMMA expr)* > make_expr_list
    / "" > empty_expr_list

  fn make_expr_list(first: Expr, rest: Vec<Expr>) -> Vec<Expr> {
    extend_front(first, rest)
  }

  fn empty_expr_list() -> Vec<Expr> {
    vec![]
  }

  entailment = stream_var ENTAILMENT expr > make_entailment_rel

  fn make_entailment_rel(left: StreamVar, right: Expr) -> EntailmentRel {
    EntailmentRel {
      left: left,
      right: right
    }
  }

  stream_var = PRE* identifier (LBRACKET list_stream_var RBRACKET)? > make_stream_var
  list_stream_var = stream_var (COMMA stream_var)* > concat_list_stream_var

  fn make_stream_var(past: Vec<()>, name: String, args: Option<Vec<StreamVar>>) -> StreamVar {
    StreamVar {
      name: name,
      past: past.len(),
      args: args.unwrap_or(vec![])
    }
  }

  fn concat_list_stream_var(first: StreamVar, rest: Vec<StreamVar>) -> Vec<StreamVar> {
    extend_front(first, rest)
  }

  var = identifier (LBRACKET list_var RBRACKET)? > make_var
  list_var = var (COMMA var)* > concat_list_var

  fn make_var(name: String, args: Option<Vec<Var>>) -> Var {
    Var {
      name: name,
      args: args.unwrap_or(vec![])
    }
  }

  fn concat_list_var(first: Var, rest: Vec<Var>) -> Vec<Var> {
    extend_front(first, rest)
  }

  spacetime
    = WORLD_LINE > world_line
    / SINGLE_TIME > single_time
    / SINGLE_SPACE > single_space
    / identifier > location_spacetime

  fn world_line() -> Spacetime { Spacetime::WorldLine }
  fn single_time() -> Spacetime { Spacetime::SingleTime }
  fn single_space() -> Spacetime { Spacetime::SingleSpace }
  fn location_spacetime(loc: String) -> Spacetime { Spacetime::Location(loc) }

  identifier = !digit !(keyword !ident_char) ident_char+ spacing > to_string
  ident_char = ["a-zA-Z0-9_"]

  number = digits > make_number
  digits = digit+ (UNDERSCORE* digit)* > concat
  digit = ["0-9"]

  // TODO: proper escape mechanism
  string_literal = "\"" (!"\"" .)* "\"" > to_string

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
  TRANSIENT = "transient" kw_tail -> ()
  PRE = "pre" kw_tail -> ()
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
  LT = "<" !"-" spacing
  GT = ">" spacing

  spacing = [" \n\r\t"]* -> (^)

  // Java keyword
  java_kw = "new"
  NEW = "new" kw_tail

  fn to_string(raw_text: Vec<char>) -> String {
    raw_text.into_iter().collect()
  }

  fn concat(mut x: Vec<char>, y: Vec<char>) -> Vec<char> {
    x.extend(y.into_iter());
    x
  }

  fn make_number(raw_number: Vec<char>) -> u64 {
    match u64::from_str(&*to_string(raw_number)).ok() {
      Some(x) => x,
      None => panic!("int literal is too large")
    }
  }

  fn extend_front<T>(first: T, rest: Vec<T>) -> Vec<T> {
    let mut r = vec![first];
    r.extend(rest.into_iter());
    r
  }
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
    let state = bonsai::recognize_program("
      let domains in world_line = VarStore.bottom();
      let constraints in world_line = ConstraintStore.bottom();
      let consistent: FlatLattice<Consistent> in single_time = FlatLattice.bottom();

      fn first_fail_middle() {
        let var in single_space = new FirstFail();
        let val in single_space = new IntDomainMin();
        loop {
          when consistent |= Consistent.Unknown {
            let x: IntVar in domains = var.getVariable(domains.vars());
            let mid: int in single_time = val.selectValue(domains[x]);
            space
            || constraints[domains] <- x.gt(mid);
            || constraints[domains] <- x.lte(mid);
            end
          }
          pause;
        }
      }

      fn model() {
        let queen1: IntVar = domains <- new IntDomain(0,1);
        let queen2: IntVar = domains <- new IntDomain(0,1);

        constraints[domains] <- queen1.ne(queen2);
        let fake in single_time = new Fake(pre pre queens1[pre queens, queen2], new Object());
      }

      fn propagation() {
        loop {
          consistent <- Solver.propagate(constraints, domains);
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
