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
  use jast::*;

  program = java_header java_class > make_java_program

  fn make_java_program(header: String, class_name: String,
    items: Vec<Item>) -> Program
  {
    Program {
      header: header,
      class_name: class_name,
      items: items
    }
  }

  java_header = (!PUBLIC .)* > to_string

  java_class = PUBLIC CLASS identifier IMPLEMENTS EXECUTABLE LBRACE item+ RBRACE

  item
    = spacetime_attribute
    / PROC identifier java_param_list block > make_process_item
    / java_method
    / java_attr
    / java_constructor

  spacetime_attribute = (CHANNEL->())? spacetime_var > make_spacetime_attribute

  spacetime_var = spacetime java_ty identifier (EQ bottom_expr)? SEMI_COLON > make_spacetime_var

  bottom_expr = expr > some_expr
              / BOT > make_bottom_expr

  fn some_expr(expr: Expr) -> Option<Expr> { Some(expr) }
  fn make_bottom_expr() -> Option<Expr> { None }

  fn make_spacetime_attribute(is_channel: Option<()>, var: LetBinding) -> Item {
    Item::Attribute(
      ModuleAttribute {
        var: var,
        is_channel: is_channel.is_some(),
      }
    )
  }

  fn make_spacetime_var(spacetime: Spacetime, var_ty: JType,
    var_name: String, expr: Option<Option<Expr>>) -> LetBinding
  {
    let expr = match expr {
      Some(Some(expr)) => expr,
      None | Some(None) => Expr::Bottom(var_ty.clone())
    };
    LetBinding::new(var_name, var_ty, spacetime, expr)
  }

  fn make_process_item(name: String, params: JParameters, body: Stmt) -> Item {
    Item::Proc(Process::new(name, params, body))
  }

  java_method
    = java_visibity (STATIC->())? java_ty identifier java_param_list java_block kw_tail > make_java_method

  java_constructor
    = java_visibity identifier java_param_list java_block kw_tail > make_java_constructor

  java_attr
    = java_visibity (STATIC->())? java_ty identifier (EQ java_expr)? SEMI_COLON > make_java_attr

  fn make_java_attr(visibility: JVisibility, is_static: Option<()>,
    ty: JType, name: String, expr: Option<Expr>) -> Item
  {
    Item::JavaAttr(
      JAttribute {
        visibility: visibility,
        is_static: is_static.is_some(),
        ty: ty,
        name: name,
        expr: expr
      }
    )
  }

  fn make_java_method(visibility: JVisibility, is_static: Option<()>,
    return_ty: JType, name: String,
    parameters: JParameters, body: JavaBlock) -> Item
  {
    let decl = JMethod {
      visibility: visibility,
      is_static: is_static.is_some(),
      return_ty: return_ty,
      name: name,
      parameters: parameters,
      body: body
    };
    Item::JavaMethod(decl)
  }


  fn make_java_constructor(visibility: JVisibility, name: String,
    parameters: JParameters, body: JavaBlock) -> Item
  {
    let decl = JConstructor {
      visibility: visibility,
      name: name,
      parameters: parameters,
      body: body
    };
    Item::JavaConstructor(decl)
  }

  java_param_list
    = &LPAREN (!")" .)* ")" blanks > make_java_param_list

  fn make_java_param_list(mut raw_list: Vec<char>,
    blanks: Vec<char>) -> JParameters
  {
    raw_list.push(')');
    raw_list.extend(blanks.into_iter());
    to_string(raw_list)
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

  java_block = "{" java_inside_block "}" > make_java_block
  java_inside_block = ((!"{" !"}" .)+ > to_string / java_block)*

  fn make_java_block(inner_blocks: Vec<JavaBlock>) -> JavaBlock {
    let mut res = extend_front(String::from("{"), inner_blocks);
    res.push(String::from("}"));
    res.iter().flat_map(|e| e.chars()).collect()
  }

  sequence = stmt+ > make_seq

  fn make_seq(stmts: Vec<Stmt>) -> Stmt {
    Stmt::Seq(stmts)
  }

  block = LBRACE sequence RBRACE

  stmt
    = PAR BARBAR? stmt (BARBAR stmt)* END > make_par
    / SPACE BARBAR? stmt (BARBAR stmt)* END > make_space
    / spacetime_var > make_let_stmt
    / spacetime_var_in_store
    / WHEN entailment block > make_when
    / PAUSE SEMI_COLON > make_pause
    / TRAP identifier block > make_trap
    / EXIT identifier SEMI_COLON > make_exit
    / LOOP block > make_loop
    / TILDE java_call_expr SEMI_COLON > make_java_call_stmt
    / var LEFT_ARROW expr SEMI_COLON > make_tell
    / identifier list_args SEMI_COLON > make_proc_call
    / block

  fn make_par(first: Stmt, rest: Vec<Stmt>) -> Stmt {
    Stmt::Par(extend_front(first, rest))
  }

  fn make_space(first: Stmt, rest: Vec<Stmt>) -> Stmt {
    Stmt::Space(extend_front(first, rest))
  }

  spacetime_var_in_store =
    java_ty identifier EQ identifier LEFT_ARROW expr SEMI_COLON > make_let_in_store_stmt

  fn make_let_in_store_stmt(loc_ty: JType, location: String,
    store: String, expr: Expr) -> Stmt
  {
    let binding = LetBinding::new(
      location, loc_ty, Spacetime::SingleTime, expr);
    Stmt::LetInStore(
      LetInStoreStmt::imperative(binding, store))
  }

  fn make_let_stmt(var: LetBinding) -> Stmt {
    Stmt::Let(LetStmt::imperative(var))
  }

  fn make_when(entailment: EntailmentRel, body: Stmt) -> Stmt {
    Stmt::When(entailment, Box::new(body))
  }

  fn make_pause() -> Stmt {
    Stmt::Pause
  }

  fn make_trap(name: String, body: Stmt) -> Stmt {
    Stmt::Trap(name, Box::new(body))
  }

  fn make_exit(name: String) -> Stmt {
    Stmt::Exit(name)
  }

  fn make_loop(body: Stmt) -> Stmt {
    Stmt::Loop(Box::new(body))
  }

  fn make_java_call_stmt(java_call: Expr) -> Stmt {
    Stmt::FnCall(java_call)
  }

  fn make_tell(var: Var, expr: Expr) -> Stmt {
    Stmt::Tell(var, expr)
  }

  fn make_proc_call(process: String, args: Vec<Expr>) -> Stmt {
    Stmt::ProcCall(process, args)
  }

  expr
    = java_expr
    / stream_var > make_stream_var_expr

  fn make_stream_var_expr(var: StreamVar) -> Expr { Expr::Variable(var) }

  java_ty
    = identifier java_generic_list > make_java_ty

  fn make_java_ty(name: String, generics: Vec<JType>) -> JType {
    JType {
      name: name,
      generics: generics
    }
  }

  java_generic_list
    = LT java_ty (COMMA java_ty)* GT? > make_generic_list
    / "" > empty_generic_list

  fn make_generic_list(first: JType, rest: Vec<JType>) -> Vec<JType> {
    extend_front(first, rest)
  }

  fn empty_generic_list() -> Vec<JType> {
    vec![]
  }

  list_args = LPAREN list_expr RPAREN

  java_expr
    = NEW java_ty list_args > java_new
    / java_call_expr
    / number > make_number_expr
    / string_literal > make_string_literal

  java_call_expr
    = identifier java_method_call+ > java_object_calls
    / java_call > java_this_call

  fn java_new(class_ty: JType, args: Vec<Expr>) -> Expr {
    Expr::JavaNew(class_ty, args)
  }

  fn java_object_calls(object: String, calls: Vec<JavaCall>) -> Expr {
    Expr::JavaObjectCall(object, calls)
  }

  fn java_this_call(java_call: JavaCall) -> Expr {
    Expr::JavaThisCall(java_call)
  }

  fn make_number_expr(n: u64) -> Expr { Expr::Number(n) }
  fn make_string_literal(lit: String) -> Expr { Expr::StringLiteral(lit) }

  java_call = identifier list_args > make_java_method_call
  java_method_call = DOT identifier (list_args)? > make_java_property

  fn make_java_method_call(name: String, args: Vec<Expr>) -> JavaCall {
    make_java_call(name, false, args)
  }

  fn make_java_property(name: String, args: Option<Vec<Expr>>) -> JavaCall {
    make_java_call(name, args.is_none(), args.unwrap_or(vec![]))
  }

  fn make_java_call(property: String, is_attribute: bool, args: Vec<Expr>) -> JavaCall {
    JavaCall {
      property: property,
      is_attribute: is_attribute,
      args: args
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

  fn world_line() -> Spacetime { Spacetime::WorldLine }
  fn single_time() -> Spacetime { Spacetime::SingleTime }
  fn single_space() -> Spacetime { Spacetime::SingleSpace }

  java_visibity
    = PUBLIC > java_public
    / PRIVATE > java_private
    / PROTECTED > java_protected

  fn java_public() -> JVisibility { JVisibility::Public }
  fn java_private() -> JVisibility { JVisibility::Private }
  fn java_protected() -> JVisibility { JVisibility::Protected }

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
    / "single_time" / "single_space" / "bot" / "top" / "channel" / java_kw
  kw_tail = !ident_char spacing

  LET = "let" kw_tail
  PROC = "proc" kw_tail
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
  CHANNEL = "channel" kw_tail

  // Java keyword
  java_kw
    = "new" / "private" / "public" / "class"
    / "implements" / "Executable" / "static"
    / "protected"
  NEW = "new" kw_tail
  PRIVATE = "private" kw_tail
  PUBLIC = "public" kw_tail
  PROTECTED = "protected" kw_tail
  CLASS = "class" kw_tail
  IMPLEMENTS = "implements" kw_tail
  EXECUTABLE = "Executable" kw_tail
  STATIC = "static" kw_tail

  UNDERSCORE = "_"
  TILDE = "~"
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

  spacing = blanks -> (^)
  blanks = [" \n\r\t"]*

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
    let state = bonsai::recognize_program(r#"
      public class Test implements Executable
      {
        proc test() {
          IntVar queen1 = domains <- new IntDomain(0,1);
          IntVar queen2 = domains <- new IntDomain(0,1);

          constraints <- new AllDifferent(domains.vars(), "DEFAULT");
          ~printVariables(domains);

          constraints <- queen1.ne(queen2);
          single_time Fake fake = new Fake(pre pre queens1[pre queens, queen2], new Object());
        }

        public Test(int i, Integer x) {
          this.m1 = i + x;
        }

        public void test1() {}
        protected void test2() {}
        private void test3() {}

        private static int s1;
        public static int s2 = 0;
        protected static int s3;

        private int m1;
        public int m2 = 0;
        protected int m3;

        single_time int x1 = bot;
        single_space int x2 = 0;
        world_line int x3 = bot;

        channel single_time int c1 = bot;
        channel single_space int c2 = 0;
        channel world_line int c3 = bot;
      }
     "#.into_state());
    let result = state.into_result();
    println!("{:?}", result);
    match result {
      Success(_) => (),
      Partial(_, _)
    | Failure(_) => assert!(false)
    };
  }
}
