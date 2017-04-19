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

use driver::module_file::ModuleFile;
use std::fmt::{Display, Error, Formatter};
use std::cmp::PartialEq;
pub use syntex_pos::Span;
pub use syntex_syntax::codemap::{mk_sp, DUMMY_SP};
pub use syntex_errors::Level;

#[derive(Clone, Debug, PartialEq)]
pub struct CompilerDiagnostic {
  pub level: Level,
  pub code: String,
  pub line: usize,
  pub column: usize
}

impl CompilerDiagnostic {
  pub fn new(level: String, code: String, line: usize, column: usize) -> Self {
    let level = CompilerDiagnostic::from_string_level(level);
    CompilerDiagnostic {
      level: level,
      code: code,
      line: line,
      column: column
    }
  }

  fn from_string_level(level: String) -> Level {
    if level == "fatal" { Level::Fatal }
    else if level == "error" { Level::Error }
    else if level == "warning" { Level::Warning }
    else if level == "note" { Level::Note }
    else if level == "help" { Level::Help }
    else {
      panic!(format!("Level `{}` not supported in compiler test attribute.", level));
    }
  }
}

impl Display for CompilerDiagnostic
{
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    fmt.write_fmt(format_args!("{}:{}:{}:{}", self.level,
      self.code, self.line, self.column))
  }
}

#[derive(Clone, Debug)]
pub struct Crate<Host> {
  pub modules: Vec<Module<Host>>,
}

impl<Host> Crate<Host> where Host: Clone {
  pub fn new() -> Self {
    Crate {
      modules: vec![],
    }
  }

  pub fn find_mod_by_name(&self, name: String) -> Option<Module<Host>> {
    self.modules.iter()
      .find(|m| m.file.mod_name() == name).cloned()
  }
}

#[derive(Clone, Debug)]
pub struct Module<Host> {
  pub fields: Vec<ModuleField>,
  pub processes: Vec<Process>,
  pub file: ModuleFile,
  pub host: Host
}

impl<Host> Module<Host> {
  pub fn ref_fields(&self) -> Vec<Binding> {
    self.fields.iter()
      .filter(|a| a.is_ref)
      .cloned()
      .map(|a| a.binding)
      .collect()
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleField {
  pub visibility: JVisibility,
  pub binding: Binding,
  pub is_ref: bool,
  pub span: Span
}

#[derive(Clone, Debug)]
pub struct Program {
  pub header: String,
  pub expected_diagnostics: Vec<CompilerDiagnostic>,
  pub package: FQN,
  pub imports: Vec<JImport>,
  pub class_name: String,
  pub interfaces: Vec<JType>,
  pub items: Vec<Item>,
  pub span: Span
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Item {
  Field(ModuleField),
  Proc(Process),
  JavaMethod(JMethod),
  JavaField(JField),
  JavaConstructor(JConstructor),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Process {
  pub visibility: JVisibility,
  pub name: String,
  pub params: JParameters,
  pub body: Stmt,
  pub span: Span
}

impl Process {
  pub fn new(span: Span, vis: JVisibility, name: String,
   params: JParameters, body: Stmt) -> Self
  {
    Process {
      visibility: vis,
      name: name,
      params: params,
      body: body,
      span: span
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stmt {
  pub node: StmtKind,
  pub span: Span
}

impl Stmt {
  pub fn new(span: Span, node: StmtKind) -> Self {
    Stmt {
      node: node,
      span: span
    }
  }

  pub fn field(binding: Binding) -> Self {
    Stmt::new(binding.span,
      StmtKind::Let(LetStmt::field(binding)))
  }

  pub fn seq(seq: Vec<Stmt>) -> Self {
    assert!(seq.len() > 0, "Try to create an empty sequence");
    Stmt::new(
      mk_sp(seq.first().unwrap().span.lo, seq.last().unwrap().span.hi),
      StmtKind::Seq(seq))
  }

  pub fn is_nothing(&self) -> bool {
    match &self.node {
      &StmtKind::Nothing => true,
      _ => false
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Stmt::new(DUMMY_SP, StmtKind::example())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StmtKind {
  Seq(Vec<Stmt>),
  Par(Vec<Stmt>),
  Space(Vec<Stmt>),
  Let(LetStmt),
  When(Condition, Box<Stmt>),
  Suspend(Condition, Box<Stmt>),
  Tell(StreamVar, Expr),
  Pause,
  PauseUp,
  Stop,
  Trap(String, Box<Stmt>),
  Exit(String),
  Loop(Box<Stmt>),
  ProcCall(String, Vec<Expr>),
  FnCall(Expr),
  ModuleCall(RunExpr),
  Universe(Box<Stmt>),
  Nothing
}

impl StmtKind {
  #[allow(dead_code)]
  pub fn example() -> Self {
    StmtKind::Tell(StreamVar::example(), Expr::example())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunExpr {
  pub module_path: VarPath,
  pub process: String,
  pub span: Span
}

impl RunExpr {
  pub fn main(span: Span, module_path: VarPath) -> Self {
    RunExpr::new(span, module_path, String::from("execute"))
  }

  pub fn new(span: Span, module_path: VarPath, process: String) -> Self {
    RunExpr {
      module_path: module_path,
      process: process,
      span: span
    }
  }

  pub fn to_expr(mut self) -> Expr {
    let head_var = self.module_path.properties.remove(0);
    let mut jcalls = self.module_path.to_java_calls();
    jcalls.push(JavaCall::empty_method(DUMMY_SP, self.process));
    Expr::new(self.span, ExprKind::JavaObjectCall(head_var, jcalls))
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LetStmt {
  pub binding: Binding,
  pub body: Box<Stmt>,
  pub is_field: bool,
  pub span: Span,
}

impl LetStmt {
  pub fn local(span: Span, binding: Binding, body: Box<Stmt>) -> Self {
    LetStmt {
      binding: binding,
      body: body,
      is_field: false,
      span: span
    }
  }

  pub fn field(binding: Binding) -> Self {
    let mut stmt = Self::imperative(binding);
    stmt.is_field = true;
    stmt
  }

  pub fn imperative(binding: Binding) -> Self {
    LetStmt::local(binding.span, binding, Box::new(Stmt::new(DUMMY_SP, StmtKind::Nothing)))
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Binding {
  pub name: String,
  pub spacetime: Spacetime,
  pub ty: JType,
  pub expr: Expr,
  pub span: Span
}

impl Binding
{
  pub fn new(span: Span, name: String,
    spacetime: Spacetime, ty: JType, expr: Expr) -> Self
  {
    Binding {
      name: name,
      spacetime: spacetime,
      ty: ty,
      expr: expr,
      span: span
    }
  }

  pub fn is_transient(&self) -> bool {
    self.spacetime.is_transient()
  }

  pub fn is_module(&self) -> bool {
    self.spacetime == Spacetime::Product
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Binding::new(DUMMY_SP, String::from("<name>"), Spacetime::example(),
      JType::example(), Expr::example())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Condition {
  Entailment(EntailmentRel),
  MetaEntailment(MetaEntailmentRel)
}

impl Condition {
  pub fn unwrap(self) -> EntailmentRel {
    match self {
      Condition::Entailment(rel) => rel,
      Condition::MetaEntailment(meta) => meta.left
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntailmentRel {
  pub left: StreamVar,
  pub right: Expr,
  pub strict: bool,
  pub span: Span
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaEntailmentRel {
  pub left: EntailmentRel,
  pub right: bool,
  pub span: Span
}

/// A variable path can be `x`, `m.x`, `m.m2.y`,... where `m` and `m2` must be checked to be module.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct VarPath {
  pub properties: Vec<String>,
  pub span: Span
}

impl VarPath {
  pub fn new(span: Span, properties: Vec<String>) -> Self {
    VarPath {
      properties: properties,
      span: span
    }
  }
  pub fn to_java_calls(&self) -> Vec<JavaCall> {
    self.properties.clone().into_iter()
      .map(|p| JavaCall::field(DUMMY_SP, p))
      .collect()
  }

  pub fn name(&self) -> String {
    self.properties.last().unwrap().clone()
  }
}

impl Display for VarPath {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let mut i = 0;
    while i < self.properties.len() - 1 {
      fmt.write_fmt(format_args!("{}.", self.properties[i]))?;
      i += 1;
    }
    fmt.write_str(self.properties[i].as_str())
  }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct StreamVar {
  pub path: VarPath,
  pub past: usize,
  pub args: Vec<StreamVar>,
  pub span: Span
}

impl StreamVar {
  pub fn new(span: Span, path: VarPath, args: Vec<StreamVar>, past: usize) -> Self {
    StreamVar {
      path: path,
      past: past,
      args: args,
      span: span
    }
  }
  pub fn simple(span: Span, name: String) -> Self {
    Self::present(span, VarPath::new(span, vec![name]), vec![])
  }

  pub fn present(span: Span, path: VarPath, args: Vec<StreamVar>) -> Self {
    Self::new(span, path, args, 0)
  }

  pub fn name(&self) -> String {
    self.path.name()
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Self::simple(DUMMY_SP, String::from("x"))
  }
}

/// The spacetime of a variable describes how it evolves in each instant. For `WorldLine` and `SingleSpace` we can additional set a boolean to `true` if the variable is transient (i.e. its value is re-initialized to bottom between each instant). The `Product` variant is used for records where variables have fields with various spacetime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spacetime {
  WorldLine(bool),
  SingleSpace(bool),
  SingleTime,
  Product
}

impl Spacetime {
  pub fn is_transient(self) -> bool {
    use self::Spacetime::*;
    match self {
      WorldLine(transient)
    | SingleSpace(transient) => transient,
      _ => false
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Spacetime::SingleSpace(false)
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expr {
  pub node: ExprKind,
  pub span: Span
}

impl Expr {
  pub fn new(span: Span, node: ExprKind) -> Self {
    Expr {
      node: node,
      span: span
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Expr::new(DUMMY_SP, ExprKind::example())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExprKind {
  JavaNew(JType, Vec<Expr>),
  JavaObjectCall(String, Vec<JavaCall>),
  JavaThisCall(JavaCall),
  Boolean(bool),
  Number(u64),
  StringLiteral(String),
  Variable(StreamVar),
  Bottom(JType)
}

impl ExprKind {
  pub fn is_bottom(&self) -> bool {
    match self {
      &ExprKind::Bottom(_) => true,
      _ => false
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    ExprKind::Variable(StreamVar::simple(DUMMY_SP, String::from("<expr>")))
  }
}

/// Java fully qualified name (FQN).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FQN {
  pub names: Vec<String>,
  pub span: Span
}

impl FQN {
  pub fn new(span: Span, names: Vec<String>) -> Self {
    FQN {
      names: names,
      span: span
    }
  }
}

impl Display for FQN {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let mut i = 0;
    while i < self.names.len() - 1 {
      fmt.write_fmt(format_args!("{}.", self.names[i]))?;
      i += 1;
    }
    fmt.write_str(self.names[i].as_str())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JImport {
  fqn: FQN,
  import_all: bool,
  span: Span
}

impl JImport {
  pub fn new(span: Span, fqn: FQN, import_all: bool) -> Self {
    JImport {
      fqn: fqn,
      import_all: import_all,
      span: span
    }
  }
}

impl Display for JImport
{
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    formatter.write_fmt(format_args!("{}", self.fqn))?;
    if self.import_all {
      formatter.write_str(".*")?;
    }
    Ok(())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JClass {
  pub header: String,
  pub package: FQN,
  pub imports: Vec<JImport>,
  pub class_name: String,
  pub interfaces: Vec<JType>,
  pub java_methods: Vec<JMethod>,
  pub java_fields: Vec<JField>,
  pub java_constructors: Vec<JConstructor>,
}

impl JClass {
  pub fn new(header: String, package: FQN, imports: Vec<JImport>,
    class_name: String, interfaces: Vec<JType>) -> Self
  {
    JClass {
      header: header,
      package: package,
      imports: imports,
      class_name: class_name,
      interfaces: interfaces,
      java_methods: vec![],
      java_fields: vec![],
      java_constructors: vec![]
    }
  }
}

pub type JCrate = Crate<JClass>;
pub type JModule = Module<JClass>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JMethod {
  pub visibility: JVisibility,
  pub is_static: bool,
  pub return_ty: JType,
  pub name: String,
  pub parameters: JParameters,
  pub body: JavaBlock,
  pub span: Span
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JConstructor {
  pub visibility: JVisibility,
  pub name: String,
  pub parameters: JParameters,
  pub body: JavaBlock,
  pub span: Span
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JField {
  pub visibility: JVisibility,
  pub is_static: bool,
  pub is_final: bool,
  pub ty: JType,
  pub name: String,
  pub expr: Option<Expr>,
  pub span: Span
}

pub type JavaBlock = String;
pub type JParameters = String;

#[derive(Clone, Debug, Eq)]
pub struct JType {
  pub name: String,
  pub generics: Vec<JType>,
  pub is_array: bool,
  pub span: Span
}

impl JType {
  pub fn simple(span: Span, name: String) -> Self {
    JType {
      name: name,
      generics: vec![],
      is_array: false,
      span: span
    }
  }

  pub fn example() -> Self {
    JType::simple(DUMMY_SP, String::from("<Java type>"))
  }
}

impl PartialEq for JType
{
  fn eq(&self, other: &JType) -> bool {
    self.name == other.name &&
    self.generics == other.generics &&
    self.is_array == other.is_array
  }
}

impl Display for JType
{
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    formatter.write_fmt(format_args!("{}", self.name))?;
    if !self.generics.is_empty() {
      let mut generics_str = String::from("<");
      for generic in &self.generics {
        generics_str.push_str(format!("{}, ", generic).as_str());
      }
      // Remove the extra ", " characters.
      generics_str.pop();
      generics_str.pop();
      formatter.write_fmt(format_args!("{}>", generics_str))?;
    }
    if self.is_array {
      formatter.write_str("[]")?;
    }
    Ok(())
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JVisibility {
  Public,
  Protected,
  Private,
}

impl Display for JVisibility {
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    use self::JVisibility::*;
    match self {
      &Public => formatter.write_str("public"),
      &Protected => formatter.write_str("protected"),
      &Private => formatter.write_str("private"),
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JavaCall {
  pub property: String, // can be a field or a method.
  pub is_field: bool,
  pub args: Vec<Expr>,
  pub span: Span
}

impl JavaCall {
  pub fn empty_method(span: Span, name: String) -> Self {
    JavaCall {
      property: name,
      is_field: false,
      args: vec![],
      span: span
    }
  }

  pub fn field(span: Span, name: String) -> Self {
    JavaCall {
      property: name,
      is_field: true,
      args: vec![],
      span: span
    }
  }
}
