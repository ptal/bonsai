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
  use oak_runtime::file_map_stream::FileMapStream;

  type Stream<'a> = FileMapStream<'a>;

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
    = module_attribute
    / java_visibility? PROC identifier java_param_list block > make_process_item
    / java_method
    / java_attr
    / java_constructor

  module_attribute = java_visibility? (CHANNEL->())? let_binding > make_module_attribute

  expr_or_bot = expr > some_expr
              / BOT > make_bottom_expr

  fn some_expr(expr: Expr) -> Option<Expr> { Some(expr) }
  fn make_bottom_expr() -> Option<Expr> { None }

  fn make_module_attribute(visibility: Option<JVisibility>, is_channel: Option<()>,
    mut binding: LetBinding) -> Item
  {
    binding.base_mut().is_module_attr = true;
    Item::Attribute(
      ModuleAttribute {
        visibility: visibility.unwrap_or(JVisibility::Private),
        binding: binding,
        is_channel: is_channel.is_some(),
      }
    )
  }

  fn make_process_item(visibility: Option<JVisibility>, name: String,
    params: JParameters, body: Stmt) -> Item
  {
    Item::Proc(Process::new(visibility.unwrap_or(JVisibility::Private),
      name, params, body))
  }

  java_method
    = java_visibility (STATIC->())? !PROC java_ty identifier java_param_list java_block kw_tail > make_java_method

  java_constructor
    = java_visibility identifier java_param_list java_block kw_tail > make_java_constructor

  java_attr
    = java_visibility (STATIC->())? java_ty identifier (EQ java_expr)? SEMI_COLON > make_java_attr

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
    / let_binding > make_let_stmt
    / WHEN condition block > make_when
    / (.. PAUSE) SEMI_COLON > make_pause
    / TRAP identifier block > make_trap
    / EXIT identifier SEMI_COLON > make_exit
    / LOOP block > make_loop
    / TILDE java_call_expr SEMI_COLON > make_java_call_stmt
    / RUN run_expr SEMI_COLON > make_run
    / var LEFT_ARROW expr SEMI_COLON > make_tell
    / identifier list_args SEMI_COLON > make_proc_call
    / block

  fn make_par(first: Stmt, rest: Vec<Stmt>) -> Stmt {
    Stmt::Par(extend_front(first, rest))
  }

  fn make_space(first: Stmt, rest: Vec<Stmt>) -> Stmt {
    Stmt::Space(extend_front(first, rest))
  }

  fn make_let_stmt(binding: LetBinding) -> Stmt {
    Stmt::Let(LetStmt::imperative(binding))
  }

  let_binding
    = spacetime TRANSIENT? let_binding_base > make_spacetime_binding
    / MODULE let_binding_base > make_module_binding
    / java_ty identifier EQ var_path LEFT_ARROW expr SEMI_COLON > make_let_in_store_binding

  let_binding_base = java_ty identifier (EQ expr_or_bot)? SEMI_COLON > make_let_binding

  fn make_let_binding(var_ty: JType,
    var_name: String, expr: Option<Option<Expr>>) -> LetBindingBase
  {
    let expr = match expr {
      Some(Some(expr)) => expr,
      None | Some(None) => Expr::Bottom(var_ty.clone())
    };
    LetBindingBase::new(var_name, var_ty, expr)
  }

  fn make_spacetime_binding(spacetime: Spacetime, is_transient: Option<()>,
    binding: LetBindingBase) -> LetBinding
  {
    LetBinding::Spacetime(LetBindingSpacetime::new(binding, spacetime, is_transient.is_some()))
  }

  fn make_module_binding(binding: LetBindingBase) -> LetBinding {
    LetBinding::Module(LetBindingModule::new(binding))
  }

  fn make_let_in_store_binding(loc_ty: JType, location: String,
    store: VarPath, expr: Expr) -> LetBinding
  {
    let binding = LetBindingBase::new(location, loc_ty, expr);
    LetBinding::InStore(
      LetBindingInStore::new(binding, store))
  }

  fn make_when(condition: Condition, body: Stmt) -> Stmt {
    Stmt::When(condition, Box::new(body))
  }

  fn make_pause(sp: Span) -> Stmt {
    Stmt::Pause(sp)
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

  fn make_tell(var: StreamVar, expr: Expr) -> Stmt {
    Stmt::Tell(var, expr)
  }

  fn make_proc_call(process: String, args: Vec<Expr>) -> Stmt {
    Stmt::ProcCall(process, args)
  }

  fn make_run(run_expr: RunExpr) -> Stmt {
    Stmt::ModuleCall(run_expr)
  }

  run_expr
    = var_path DOT identifier parens > make_run_expr
    / var_path > make_identifier_run

  parens = LPAREN RPAREN

  fn make_identifier_run(module_path: VarPath) -> RunExpr {
    RunExpr::main(module_path)
  }

  fn make_run_expr(module_path: VarPath, process: String) -> RunExpr {
    RunExpr::new(module_path, process)
  }

  expr
    = java_expr
    / stream_var > make_stream_var_expr

  fn make_stream_var_expr(var: StreamVar) -> Expr { Expr::Variable(var) }

  java_ty
    = identifier java_generic_list (LBRACKET RBRACKET -> ())? > make_java_ty

  fn make_java_ty(name: String, generics: Vec<JType>, is_array: Option<()>) -> JType {
    JType {
      name: name,
      generics: generics,
      is_array: is_array.is_some()
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
    = java_new_expr
    / java_call_expr
    / boolean > make_boolean_expr
    / number > make_number_expr
    / string_literal > make_string_literal

  java_new_expr = NEW java_ty list_args > java_new

  java_call_expr
    = identifier java_property_call+ > java_object_calls
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

  fn make_boolean_expr(b: bool) -> Expr { Expr::Boolean(b) }
  fn make_number_expr(n: u64) -> Expr { Expr::Number(n) }
  fn make_string_literal(lit: String) -> Expr { Expr::StringLiteral(lit) }

  java_call = identifier list_args > make_java_method_call
  java_property_call = DOT identifier (list_args)? > make_java_property

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

  condition
    = meta_entailment > make_meta_condition
    / entailment > make_condition

  meta_entailment = LPAREN entailment RPAREN ENTAILMENT boolean > make_meta_entailment_rel
  entailment = stream_var ENTAILMENT expr > make_entailment_rel

  fn make_meta_condition(entailment: MetaEntailmentRel) -> Condition {
    Condition::MetaEntailment(entailment)
  }
  fn make_condition(entailment: EntailmentRel) -> Condition {
    Condition::Entailment(entailment)
  }

  fn make_entailment_rel(left: StreamVar, right: Expr) -> EntailmentRel {
    EntailmentRel {
      left: left,
      right: right
    }
  }

  fn make_meta_entailment_rel(left: EntailmentRel, right: bool) -> MetaEntailmentRel {
    MetaEntailmentRel {
      left: left,
      right: right
    }
  }

  var_path = identifier (DOT identifier !parens)* > make_var_path

  fn make_var_path(first: String, rest: Vec<String>) -> VarPath {
    VarPath::new(extend_front(first, rest))
  }

  stream_var = PRE* var_path (LBRACKET list_stream_var RBRACKET)? > make_stream_var
  list_stream_var = stream_var (COMMA stream_var)* > concat_list_stream_var

  fn make_stream_var(past: Vec<()>, var_path: VarPath, args: Option<Vec<StreamVar>>) -> StreamVar {
    StreamVar::new(var_path, args.unwrap_or(vec![]), past.len())
  }

  fn concat_list_stream_var(first: StreamVar, rest: Vec<StreamVar>) -> Vec<StreamVar> {
    extend_front(first, rest)
  }

  var = var_path (LBRACKET list_var RBRACKET)? > make_var
  list_var = var (COMMA var)* > concat_list_stream_var

  fn make_var(var_path: VarPath, args: Option<Vec<StreamVar>>) -> StreamVar {
    StreamVar::present(var_path, args.unwrap_or(vec![]))
  }

  spacetime
    = WORLD_LINE > world_line
    / SINGLE_TIME > single_time
    / SINGLE_SPACE > single_space

  fn world_line() -> Spacetime { Spacetime::WorldLine }
  fn single_time() -> Spacetime { Spacetime::SingleTime }
  fn single_space() -> Spacetime { Spacetime::SingleSpace }

  java_visibility
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

  boolean
    = TRUE > make_true
    / FALSE > make_false

  fn make_true() -> bool { true }
  fn make_false() -> bool { false }

  keyword
    = "let" / "fn" / "par" / "space" / "end" / "transient" / "pre" / "when"
    / "loop" / "pause" / "trap" / "exit" / "in" / "world_line"
    / "single_time" / "single_space" / "bot" / "top" / "channel" / "module"
    / "run" / "true" / "false" / java_kw
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
  MODULE = "module" kw_tail
  RUN = "run" kw_tail
  TRUE = "true" kw_tail
  FALSE = "false" kw_tail

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

  spacing = (blank+ -> () / comment)* -> (^)
  blank = [" \n\r\t"]
  blanks = blank*
  line_break = ["\r\n"]
  comment
    = oneline_comment -> (^)
    / multiline_comment -> (^)

  oneline_comment
    = SLASH_SLASH (!line_break .)* spacing

  multiline_comment
    = LCOMMENT ((!(LCOMMENT/RCOMMENT) .)+ (&LCOMMENT multiline_comment)?)* RCOMMENT

  SLASH_SLASH = "//"
  LCOMMENT = "/*" blanks
  RCOMMENT = "*/" blanks

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
        // This is a test grammar.
        /* We can do multi-line comments
           . /* even inlined comments */
           .
        */
        proc test() {
          IntVar queen1 = domains <- new IntDomain(0,1);
          IntVar queen2 = domains <- new IntDomain(0,1);

          constraints <- new AllDifferent(domains.vars(), "DEFAULT");
          ~printVariables(domains);

          constraints <- queen1.ne(queen2);
          single_time Fake fake = new Fake(pre pre queens1[pre queens, queen2], new Object());
          // Testing meta-entailment
          when (pre x |= x) |= false {
            pause;
          }
        }

        public Test(int i, Integer x) {
          this.m1 = i + x;
          module Propagation prop = new Propagation();
        }
        /* Different way of declaring the variables. */
        private Integer[] array1;

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
        private world_line int x3 = bot;

        module int x1 = bot;
        public module int x2 = 0;
        module int x3 = bot;

        channel single_time int c1 = bot;
        protected channel single_space int c2 = 0;
        channel world_line int c3 = bot;

        channel single_time transient int t1 = bot;
        single_space transient int t2 = bot;
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
