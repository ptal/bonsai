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

  /// Convention: `_os` means that the rule is not parsing trailing space (os = open space).
  ///             This is required to avoid reporting error with trailing space included.

  use std::str::FromStr;
  use ast::*;
  use syntex_pos::Span;
  use oak_runtime::file_map_stream::FileMapStream;

  type Stream<'a> = FileMapStream<'a>;

  program = .. pre_header header java_class > make_java_program

  test_annotation = HASH LBRACKET (execution_test_attr / compiler_test_attr) RBRACKET

  execution_test_attr = (mode spacing) LPAREN string_identifier_os DOT string_identifier COMMA string_literal RPAREN > make_execution_test

  compiler_test_attr = string_identifier LPAREN string_identifier COMMA number COMMA number RPAREN > make_compiler_test

  mode
    = "run" > make_false
    / "debug" > make_true

  fn make_compiler_test(level: String, code: String,
   line: u64, column: u64) -> TestAnnotation
  {
    TestAnnotation::Compiler(CompilerTest::new(level, code, line as usize, column as usize))
  }

  fn make_execution_test(filter_debug: bool, class_name: String, process_name: String, regex: String) -> TestAnnotation
  {
    // We ensure that the regex matches "exactly" the given string.
    let regex = format!("^{}$", regex);
    TestAnnotation::Execution(ExecutionTest::new(class_name, process_name, regex, filter_debug))
  }

  fn make_java_program(span: Span, pre_header: String, tests: Vec<TestAnnotation>,
   package: FQN, imports: Vec<JImport>,
   class_name: Ident, interfaces: Vec<JType>, items: Vec<Item>) -> Program
  {
    Program {
      header: pre_header,
      tests: tests,
      package: package,
      imports: imports,
      class_name: class_name,
      interfaces: interfaces,
      items: items,
      span: span
    }
  }

  pre_header = (!(test_annotation* PACKAGE) .)* > to_string

  header = test_annotation* java_package java_import*

  java_import
    = (.. IMPORT fully_qualified_name) SEMI_COLON > make_single_type_import
    / (.. IMPORT fully_qualified_name DOT STAR) SEMI_COLON > make_all_type_import

  fn make_single_type_import(span: Span, fqn: FQN) -> JImport {
    JImport::new(span, fqn, false)
  }

  fn make_all_type_import(span: Span, fqn: FQN) -> JImport {
    JImport::new(span, fqn, true)
  }

  java_package = PACKAGE fully_qualified_name SEMI_COLON

  fully_qualified_name = .. identifier (DOT identifier)* > make_fully_qualified_name

  fn make_fully_qualified_name(span: Span, first: Ident, rest: Vec<Ident>) -> FQN {
    FQN::new(span, extend_front(first, rest))
  }

  java_class = PUBLIC CLASS identifier interfaces_list LBRACE item+ RBRACE

  interfaces_list
    = IMPLEMENTS java_ty (COMMA java_ty)* > make_list_java_ty
    / "" > empty_java_ty_list

  fn make_list_java_ty(first: JType, rest: Vec<JType>) -> Vec<JType> {
    extend_front(first, rest)
  }

  fn empty_java_ty_list() -> Vec<JType> {
    vec![]
  }

  item
    = module_field
    / (.. java_visibility? proc_or_flow identifier java_param_list) EQ open_sequence > make_process_item
    / java_field
    / java_method
    / java_constructor

  proc_or_flow = (.. (PROC > make_false / FLOW > make_true))

  module_field = (.. java_visibility? (.. REF)? bonsai_binding) SEMI_COLON > make_module_field

  fn make_module_field(span: Span, visibility: Option<JVisibility>,
    is_ref: Option<Span>, binding: Binding) -> Item
  {
    Item::Field(ModuleField::bonsai_field(
      span, visibility, binding, is_ref))
  }

  fn make_process_item(span: Span, visibility: Option<JVisibility>, flow_kw_sp: Span, is_flow: bool,
    name: Ident, params: JParameters, body: Stmt) -> Item
  {
    let body = if is_flow {
      let sp = body.span.clone();
      Stmt::new(sp, make_flow(flow_kw_sp, body))
    }
    else { body };
    Item::Proc(Process::new(span, visibility, name, params, body))
  }

  java_method
    = .. java_visibility (STATIC->())? !PROC java_ty identifier java_param_list java_block kw_tail > make_java_method

  java_constructor
    = .. java_visibility identifier java_param_list java_block kw_tail > make_java_constructor

  java_field
    = (.. (FINAL->())? java_visibility? (STATIC->())? java_binding) SEMI_COLON > make_java_field

  fn make_java_field(span: Span, is_final: Option<()>, visibility: Option<JVisibility>,
    is_static: Option<()>, binding: Binding) -> Item
  {
    Item::Field(ModuleField::java_field(
      span, visibility, binding, is_static.is_some(), is_final.is_some()))
  }

  fn make_java_method(span: Span, visibility: JVisibility, is_static: Option<()>,
    return_ty: JType, name: Ident,
    parameters: JParameters, body: JavaBlock) -> Item
  {
    let decl = JMethod {
      visibility: visibility,
      is_static: is_static.is_some(),
      return_ty: return_ty,
      name: name,
      parameters: parameters,
      body: body,
      span: span
    };
    Item::JavaMethod(decl)
  }

  fn make_java_constructor(span: Span, visibility: JVisibility, name: Ident,
    parameters: JParameters, body: JavaBlock) -> Item
  {
    let decl = JConstructor {
      visibility: visibility,
      name: name,
      parameters: parameters,
      body: body,
      span: span
    };
    Item::JavaConstructor(decl)
  }

  java_param_list
    = LPAREN jparameter (COMMA jparameter)* RPAREN > make_java_param_list
    / LPAREN RPAREN > empty_param_list

  jparameter = (.. java_ty identifier_os) spacing > make_jparameter

  fn make_java_param_list(first: JParameter, rest: Vec<JParameter>) -> JParameters
  {
    extend_front(first, rest)
  }

  fn empty_param_list() -> JParameters { vec![] }

  fn make_jparameter(span: Span, java_ty: JType, name: Ident) -> JParameter {
    JParameter::new(span, java_ty, name)
  }

  java_block = "{" java_inside_block "}" > make_java_block
  java_inside_block = ((!"{" !"}" .)+ > to_string / java_block)*

  fn make_java_block(inner_blocks: Vec<JavaBlock>) -> JavaBlock {
    let mut res = extend_front(String::from("{"), inner_blocks);
    res.push(String::from("}"));
    res.iter().flat_map(|e| e.chars()).collect()
  }

  // An open sequence is a sequence in a process such as `proc f = s1; s2 end`.
  // We need a keyword `end` to be disambiguate between variable declaration and class attribute.
  open_sequence
    = (.. stmt (SEMI_COLON stmt)+ SEMI_COLON? END_OS) spacing > make_seq
    / stmt SEMI_COLON? END?

  // Sequences occurring inside a process do not need this `end` delimiter.
  close_sequence
    = (.. stmt (SEMI_COLON stmt)+) SEMI_COLON? > make_seq
    / stmt SEMI_COLON?

  fn make_seq(span: Span, first: Stmt, rest: Vec<Stmt>) -> Stmt {
    make_stmt(span, StmtKind::Seq(extend_front(first, rest)))
  }

  stmt
    = (.. stmt_kind_os) spacing > make_stmt

  stmt_kind_os
    = PAR BARBAR? close_sequence (BARBAR close_sequence)* END_OS > make_and_par
    / PAR DIAMOND? close_sequence (DIAMOND close_sequence)* END_OS > make_or_par
    / SPACE close_sequence END_OS > make_space
    / PRUNE_OS > make_prune
    / WHEN expr THEN close_sequence (ELSE close_sequence)? END_OS > make_when
    / SUSPEND WHEN expr IN close_sequence END_OS > make_suspend
    / ABORT WHEN expr IN close_sequence END_OS > make_abort
    / PAUSEUP_OS > make_pause_up
    / STOP_OS > make_stop
    / PAUSE_OS > make_pause
    / NOTHING_OS > make_nothing
    / LOOP close_sequence END_OS > make_loop
    / (.. FLOW) close_sequence END_OS > make_flow
    / UNIVERSE close_sequence END_OS > make_qf_universe
    / UNIVERSE WITH variable IN close_sequence END_OS > make_universe
    / RUN proc_call_os > make_proc_call
    / binding_os > make_let_stmt
    / variable LEFT_ARROW expr > make_tell
    / expr > make_expr_stmt

  fn make_stmt(span: Span, stmt_kind: StmtKind) -> Stmt {
    Stmt::new(span, stmt_kind)
  }

  fn make_or_par(first: Stmt, rest: Vec<Stmt>) -> StmtKind {
    StmtKind::OrPar(extend_front(first, rest))
  }

  fn make_and_par(first: Stmt, rest: Vec<Stmt>) -> StmtKind {
    StmtKind::AndPar(extend_front(first, rest))
  }

  fn make_space(branch: Stmt) -> StmtKind {
    StmtKind::Space(Box::new(branch))
  }

  fn make_prune() -> StmtKind {
    StmtKind::Prune
  }

  fn make_let_stmt(binding: Binding) -> StmtKind {
    StmtKind::Let(LetStmt::imperative(binding))
  }

  binding_os
    = bonsai_binding_os
    / java_binding_os

  bonsai_binding_os = .. kind java_ty identifier_os (spacing EQ expr)? > make_bonsai_binding
  java_binding_os = .. java_ty identifier_os (spacing EQ expr)? > make_java_binding
  bonsai_binding = bonsai_binding_os spacing
  java_binding = java_binding_os spacing

  fn make_bonsai_binding(span: Span, kind: Kind,
    ty: JType, name: Ident, expr: Option<Expr>) -> Binding
  {
    Binding::new(span, name, kind, ty, expr)
  }

  fn make_java_binding(span: Span, ty: JType, name: Ident, expr: Option<Expr>) -> Binding {
    Binding::new(span, name, Kind::Host, ty, expr)
  }

  fn make_when(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> StmtKind {
    let else_branch =
      match else_branch {
        Some(b) => b,
        None => {
          let sp = mk_sp(then_branch.span.hi, then_branch.span.hi);
          make_stmt(sp, StmtKind::Nothing)
        }
      };
    StmtKind::When(condition, Box::new(then_branch), Box::new(else_branch))
  }

  fn make_suspend(condition: Expr, body: Stmt) -> StmtKind {
    StmtKind::Suspend(SuspendStmt::new(condition, Box::new(body)))
  }

  fn make_abort(condition: Expr, body: Stmt) -> StmtKind {
    StmtKind::Abort(condition, Box::new(body))
  }

  fn make_pause() -> StmtKind {
    StmtKind::DelayStmt(Delay::new(DelayKind::Pause))
  }

  fn make_pause_up() -> StmtKind {
    StmtKind::DelayStmt(Delay::new(DelayKind::PauseUp))
  }

  fn make_stop() -> StmtKind {
    StmtKind::DelayStmt(Delay::new(DelayKind::Stop))
  }

  fn make_nothing() -> StmtKind {
    StmtKind::Nothing
  }

  fn make_loop(body: Stmt) -> StmtKind {
    StmtKind::Loop(Box::new(body))
  }

  // We desugare `flow p end` into `loop p; pause end`.
  fn make_flow(flow_kw_sp: Span, body: Stmt) -> StmtKind {
    let pause = Stmt::new(flow_kw_sp, make_pause());
    let sp = body.span.clone();
    let desugared = Stmt::new(sp, StmtKind::Seq(vec![body,pause]));
    StmtKind::Loop(Box::new(desugared))
  }

  fn make_tell(var: Variable, expr: Expr) -> StmtKind {
    StmtKind::Tell(var, expr)
  }

  fn make_expr_stmt(expr: Expr) -> StmtKind {
    StmtKind::ExprStmt(expr)
  }

  fn make_proc_call(var: Option<Variable>, process: Ident, args: Vec<Variable>) -> StmtKind {
    StmtKind::ProcCall(var, process, args)
  }

  fn make_qf_universe(body: Stmt) -> StmtKind {
    StmtKind::QFUniverse(Box::new(body))
  }

  fn make_universe(queue: Variable, body: Stmt) -> StmtKind {
    StmtKind::Universe(queue, Box::new(body))
  }

  proc_call_os = (variable DOT)? identifier LPAREN list_var RPAREN_OS

  list_var
    = variable (COMMA variable)* > make_list_var
    / "" > empty_var_list

  fn make_list_var(first: Variable, rest: Vec<Variable>) -> Vec<Variable> {
    extend_front(first, rest)
  }

  fn empty_var_list() -> Vec<Variable> {
    vec![]
  }

  java_ty
    = .. identifier java_generic_list (LBRACKET RBRACKET -> ())? > make_java_ty

  fn make_java_ty(span: Span, name: Ident, generics: Vec<JType>, is_array: Option<()>) -> JType {
    JType {
      name: name,
      generics: generics,
      is_array: is_array.is_some(),
      span: span
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

  list_expr
    = expr (COMMA expr)* > make_expr_list
    / "" > empty_expr_list

  fn make_expr_list(first: Expr, rest: Vec<Expr>) -> Vec<Expr> {
    extend_front(first, rest)
  }

  fn empty_expr_list() -> Vec<Expr> {
    vec![]
  }

  expr
    = expr_2 (binary_op expr)* > fold_left_binary_op

  // We use a boolean to distinguish `or` and `and` without creating an additional (temporary) structure.
  binary_op
    = OR > make_true
    / AND > make_false

  expr_2
    = .. NOT expr_atom > make_trilean_not_expr
    / .. expr_atom entailment_kind expr_atom > make_entailment_rel
    / expr_atom

  expr_atom
    = .. expr_atom_kind > make_expr
    / LPAREN expr RPAREN

  expr_atom_kind
    = trilean > make_trilean_expr
    / BOT > make_bottom_expr
    / TOP > make_top_expr
    / host_expr
    / variable > make_var_expr

  host_expr
    = number > make_number_expr
    / method_call > make_call
    / string_literal > make_string_literal
    / new_instance_expr > make_new_instance

  entailment_kind
    = ENTAILMENT > make_entailment_op
    / ENTAILMENT_STRICT > make_strict_entailment_op
    / EQUALITY > make_equality_op

  fn fold_left_binary_op(head: Expr, rest: Vec<(bool, Expr)>) -> Expr {
    rest.into_iter().fold(head,
      |accu, (op, expr)| {
        let lo = accu.span.lo;
        let hi = expr.span.hi;
        let expr_kind = match op {
          true => ExprKind::Or(Box::new(accu), Box::new(expr)),
          false => ExprKind::And(Box::new(accu), Box::new(expr))
        };
        make_expr(mk_sp(lo, hi), expr_kind)
      })
  }

  fn make_trilean_not_expr(span: Span, expr: Expr) -> Expr {
    make_expr(span, ExprKind::Not(Box::new(expr)))
  }

  fn make_entailment_rel(span: Span, left: Expr, op: EntailmentKind, right: Expr) -> Expr {
    match op {
      EntailmentKind::StrictEntailment => { panic!("|< is not yet implemented."); }
      EntailmentKind::Equality => { panic!("== is not yet implemented."); }
      _ => ()
    };
    let e = EntailmentRel { left, right, op };
    make_expr(span, ExprKind::Entailment(Box::new(e)))
  }

  fn make_bottom_expr() -> ExprKind { ExprKind::Bottom }
  fn make_top_expr() -> ExprKind { ExprKind::Top }

  fn make_expr(span: Span, node: ExprKind) -> Expr {
    Expr::new(span, node)
  }

  fn make_new_instance(new_instance: NewObjectInstance) -> ExprKind {
    ExprKind::NewInstance(new_instance)
  }
  fn make_call(call: MethodCall) -> ExprKind { ExprKind::Call(call) }
  fn make_trilean_expr(t: SKleene) -> ExprKind { ExprKind::Trilean(t) }
  fn make_number_expr(n: u64) -> ExprKind { ExprKind::Number(n) }
  fn make_string_literal(lit: String) -> ExprKind { ExprKind::StringLiteral(lit) }

  fn make_entailment_op() -> EntailmentKind { EntailmentKind::Entailment }
  fn make_strict_entailment_op() -> EntailmentKind { EntailmentKind::StrictEntailment }
  fn make_equality_op() -> EntailmentKind { EntailmentKind::Equality }

  new_instance_expr = (.. NEW java_ty LPAREN list_expr RPAREN_OS) spacing > make_new_object_instance

  fn make_new_object_instance(span: Span, class_ty: JType, args: Vec<Expr>) -> NewObjectInstance {
    NewObjectInstance::new(span, class_ty, args)
  }

  fn make_var_expr(variable: Variable) -> ExprKind { ExprKind::Var(variable) }

  var_path_os = .. identifier_os (spacing DOT identifier_os !(spacing LPAREN))* > make_var_path

  fn make_var_path(span: Span, first: Ident, rest: Vec<Ident>) -> VarPath {
    VarPath::new(span, extend_front(first, rest))
  }

  variable
    = (.. PRE+ var_path_os !(spacing LPAREN)) spacing > make_stream_variable
    / (.. permission? var_path_os !(spacing LPAREN)) spacing > make_variable

  fn make_stream_variable(span: Span, past: Vec<()>, path: VarPath) -> Variable {
    Variable::stream(span, path, past.len())
  }

  fn make_variable(span: Span, permission: Option<Permission>, path: VarPath) -> Variable {
    Variable::access(span, path, permission)
  }

  permission
    = READWRITE > make_readwrite_permission
    / WRITE > make_write_permission
    / READ > make_read_permission

  fn make_read_permission() -> Permission { Permission::Read }
  fn make_write_permission() -> Permission { Permission::Write }
  fn make_readwrite_permission() -> Permission { Permission::ReadWrite }

  method_call
    = (.. (THIS DOT)? fn_call_os) spacing > make_static_method_call
    / (.. variable DOT fn_call_os) spacing > make_method_call

  fn_call_os = identifier LPAREN list_expr RPAREN_OS

  fn make_static_method_call(span: Span, method: Ident, args: Vec<Expr>) -> MethodCall {
    MethodCall::new(span, None, method, args)
  }

  fn make_method_call(span: Span, target: Variable, method: Ident, args: Vec<Expr>) -> MethodCall {
    MethodCall::new(span, Some(target), method, args)
  }

  kind
    = spacetime > kind_spacetime
    / MODULE > kind_product

  spacetime
    = WORLD_LINE > world_line
    / SINGLE_TIME > single_time
    / SINGLE_SPACE > single_space

  fn world_line() -> Spacetime { Spacetime::WorldLine }
  fn single_time() -> Spacetime { Spacetime::SingleTime }
  fn single_space() -> Spacetime { Spacetime::SingleSpace }

  fn kind_product() -> Kind { Kind::Product }
  fn kind_spacetime(spacetime: Spacetime) -> Kind { Kind::Spacetime(spacetime) }

  java_visibility
    = PUBLIC > java_public
    / PRIVATE > java_private
    / PROTECTED > java_protected

  fn java_public() -> JVisibility { JVisibility::Public }
  fn java_private() -> JVisibility { JVisibility::Private }
  fn java_protected() -> JVisibility { JVisibility::Protected }

  identifier = identifier_os spacing
  identifier_os = .. string_identifier_os > to_ident

  fn to_ident(span: Span, value: String) -> Ident {
    Ident::new(span, value)
  }

  string_identifier = string_identifier_os spacing
  string_identifier_os = !digit !(keyword !ident_char) ident_char+ > to_string
  ident_char = ["a-zA-Z0-9_"]

  number = digits spacing > make_number
  digits = digit+ (UNDERSCORE* digit)* > concat
  digit = ["0-9"]

  // TODO: proper escape mechanism
  string_literal = "\"" (!"\"" .)* "\"" spacing > to_string

  fn make_true() -> bool { true }
  fn make_false() -> bool { false }

  trilean
    = KTRUE > make_ktrue
    / KFALSE > make_kfalse
    / KUNKNOWN > make_kunknown

  fn make_ktrue() -> SKleene { SKleene::True }
  fn make_kfalse() -> SKleene { SKleene::False }
  fn make_kunknown() -> SKleene { SKleene::Unknown }

  keyword
    = "proc" / "par" / "space" / "prune" / "end" / "pre" / "nothing"
    / "when" / "then" / "else"
    / "loop" / "flow" /  "pause up" / "pause" / "stop" / "in" / "world_line"
    / "single_time" / "single_space" / "bot" / "top" / "ref" / "module"
    / "readwrite" / "read" / "write"
    / "or" / "and" / "not"
    / "run" / "true" / "false" / "unknown"
    / "universe" /  "with"
    / "suspend" / "abort" / java_kw
  kw_tail = kw_tail_os spacing
  kw_tail_os = !ident_char

  END_OS = "end" kw_tail_os
  PRUNE_OS = "prune" kw_tail_os
  STOP_OS = "stop" kw_tail_os
  PAUSE_OS = "pause" kw_tail_os
  PAUSEUP_OS = "pause up" kw_tail_os
  NOTHING_OS = "nothing" kw_tail_os
  RPAREN_OS = ")" kw_tail_os

  PROC = "proc" kw_tail
  PAR = "par" kw_tail
  SPACE = "space" kw_tail
  END = "end" kw_tail
  PRE = "pre" kw_tail -> ()
  WHEN = "when" kw_tail
  ELSE = "else" kw_tail
  THEN = "then" kw_tail
  SUSPEND = "suspend" kw_tail
  ABORT = "abort" kw_tail
  LOOP = "loop" kw_tail
  FLOW = "flow" kw_tail
  IN = "in" kw_tail
  WITH = "with" kw_tail
  WORLD_LINE = "world_line" kw_tail
  SINGLE_TIME = "single_time" kw_tail
  SINGLE_SPACE = "single_space" kw_tail
  BOT = "bot" kw_tail
  TOP = "top" kw_tail
  REF = "ref" kw_tail
  MODULE = "module" kw_tail
  RUN = "run" kw_tail
  KTRUE = "true" kw_tail
  KFALSE = "false" kw_tail
  KUNKNOWN = "unknown" kw_tail
  UNIVERSE = "universe" kw_tail
  READ = "read" kw_tail
  WRITE = "write" kw_tail
  READWRITE = "readwrite" kw_tail
  OR = "or" kw_tail
  AND = "and" kw_tail
  NOT = "not" kw_tail

  // Java keyword
  java_kw
    = "new" / "private" / "public" / "class"
    / "implements" / "static"
    / "protected" / "final" / "import" / "package"
    / "this"
  NEW = "new" kw_tail
  PRIVATE = "private" kw_tail
  PUBLIC = "public" kw_tail
  PROTECTED = "protected" kw_tail
  CLASS = "class" kw_tail
  IMPLEMENTS = "implements" kw_tail
  STATIC = "static" kw_tail
  FINAL = "final" kw_tail
  PACKAGE = "package" kw_tail
  IMPORT = "import" kw_tail
  THIS = "this" kw_tail

  UNDERSCORE = "_"
  DOTDOT = ".." spacing
  SEMI_COLON = ";" spacing
  COLON = ":" spacing
  EQ = "=" spacing
  COMMA = "," spacing
  DOT = "." spacing
  BARBAR = "||" spacing
  DIAMOND = "<>" spacing
  ENTAILMENT = "|=" spacing
  ENTAILMENT_STRICT = "|<" spacing
  EQUALITY = "==" spacing
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
  HASH = "#" spacing
  STAR = "*" spacing

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

// #[cfg(test)]
// mod test
// {
//   use oak_runtime::*;
//   use oak_runtime::ParseResult::*;
//   use super::*;

//   #[test]
//   fn test_grammar()
//   {
//     let state = bonsai::recognize_program(r#"
//       public class Test
//       {
//         // This is a test grammar.
//         /* We can do multi-line comments
//            . /* even inlined comments */
//            .
//         */
//         proc test() {
//           IntVar queen1 = domains <- new IntDomain(0,1);
//           IntVar queen2 = domains <- new IntDomain(0,1);

//           constraints <- new AllDifferent(domains.vars(), "DEFAULT");
//           ~printVariables(domains);

//           constraints <- queen1.ne(queen2);
//           single_time Fake fake = new Fake(pre pre queens1[pre queens, queen2], new Object());
//           // Testing meta-entailment
//           when (pre x |= x) |= false {
//             pause;
//           }
//         }

//         public Test(int i, Integer x) {
//           this.m1 = i + x;
//           module Propagation prop = new Propagation();
//         }
//         /* Different way of declaring the variables. */
//         private Integer[] array1;

//         public void test1() {}
//         protected void test2() {}
//         private void test3() {}

//         private static int s1;
//         public static int s2 = 0;
//         protected static int s3;

//         private int m1;
//         public int m2 = 0;
//         protected int m3;

//         single_time int x1 = bot;
//         single_space int x2 = 0;
//         private world_line int x3 = bot;

//         module int x1 = bot;
//         public module int x2 = 0;
//         module int x3 = bot;

//         ref single_time int c1 = bot;
//         protected ref single_space int c2 = 0;
//         ref world_line int c3 = bot;
//       }
//      "#.into_state());
//     let result = state.into_result();
//     println!("{:?}", result);
//     match result {
//       Success(_) => (),
//       Partial(_, _)
//     | Failure(_) => assert!(false)
//     };
//   }
// }
