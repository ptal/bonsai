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
use std::ops::Deref;
use std::hash::{Hash, Hasher};
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

  pub fn find_field_by_name(&self, name: String) -> Option<ModuleField> {
    self.fields.iter()
      .find(|f| *f.binding.name == name).cloned()
  }
}

impl Module<JClass> {
  pub fn new(file: ModuleFile, ast: Program) -> Self {
    let mut module = Module {
      fields: vec![],
      processes: vec![],
      file: file,
      host: JClass::new(ast.header, ast.package, ast.imports, ast.class_name, ast.interfaces)
    };
    for item in ast.items {
      match item {
        Item::Field(field) => module.fields.push(field),
        Item::Proc(process) => module.processes.push(process),
        Item::JavaMethod(decl) => module.host.java_methods.push(decl),
        Item::JavaConstructor(decl) => module.host.java_constructors.push(decl)
      }
    }
    module
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleField {
  pub visibility: JVisibility,
  pub binding: Binding,
  pub is_ref: bool,
  pub is_static: bool,
  pub is_final: bool,
  pub span: Span
}

impl ModuleField {
  fn new(span: Span, visibility: Option<JVisibility>,
    binding: Binding, is_ref: bool, is_static: bool, is_final: bool) -> Self
  {
    ModuleField {
      visibility: visibility.unwrap_or(JVisibility::Private),
      binding: binding,
      is_ref: is_ref,
      is_static: is_static,
      is_final: is_final,
      span: span
    }
  }

  pub fn bonsai_field(span: Span, visibility: Option<JVisibility>,
    binding: Binding, is_ref: bool) -> Self
  {
    ModuleField::new(span, visibility, binding, is_ref, false, true)
  }

  pub fn java_field(span: Span, visibility: Option<JVisibility>,
    binding: Binding, is_static: bool, is_final: bool) -> Self
  {
    ModuleField::new(span, visibility, binding, false, is_static, is_final)
  }
}

#[derive(Clone, Debug)]
pub struct Program {
  pub header: String,
  pub expected_diagnostics: Vec<CompilerDiagnostic>,
  pub package: FQN,
  pub imports: Vec<JImport>,
  pub class_name: Ident,
  pub interfaces: Vec<JType>,
  pub items: Vec<Item>,
  pub span: Span
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Item {
  Field(ModuleField),
  Proc(Process),
  JavaMethod(JMethod),
  JavaConstructor(JConstructor),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Process {
  pub visibility: JVisibility,
  pub name: Ident,
  pub params: JParameters,
  pub body: Stmt,
  pub span: Span
}

impl Process {
  pub fn new(span: Span, visibility: Option<JVisibility>, name: Ident,
   params: JParameters, body: Stmt) -> Self
  {
    Process {
      visibility: visibility.unwrap_or(JVisibility::Private),
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
  When(EntailmentRel, Box<Stmt>),
  Suspend(EntailmentRel, Box<Stmt>),
  Tell(Variable, Expr),
  Pause,
  PauseUp,
  Stop,
  Trap(Ident, Box<Stmt>),
  Exit(Ident),
  Loop(Box<Stmt>),
  ProcCall(Variable, Ident),
  ExprStmt(Expr),
  Universe(Box<Stmt>),
  Nothing
}

impl StmtKind {
  #[allow(dead_code)]
  pub fn example() -> Self {
    StmtKind::Tell(Variable::example(), Expr::example())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LetStmt {
  pub binding: Binding,
  pub body: Box<Stmt>,
  pub span: Span,
}

impl LetStmt {
  pub fn local(span: Span, binding: Binding, body: Box<Stmt>) -> Self {
    LetStmt {
      binding: binding,
      body: body,
      span: span
    }
  }

  pub fn imperative(binding: Binding) -> Self {
    LetStmt::local(binding.span, binding, Box::new(Stmt::new(DUMMY_SP, StmtKind::Nothing)))
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Binding {
  pub name: Ident,
  pub uid: usize,
  pub kind: Kind,
  pub ty: JType,
  pub expr: Option<Expr>,
  pub span: Span
}

impl Binding
{
  pub fn new(span: Span, name: Ident,
    kind: Kind, ty: JType, expr: Option<Expr>) -> Self
  {
    Binding {
      name: name,
      uid: 0,
      kind: kind,
      ty: ty,
      expr: expr,
      span: span
    }
  }

  pub fn is_transient(&self) -> bool {
    self.kind.is_transient()
  }

  pub fn is_module(&self) -> bool {
    self.kind == Kind::Product
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Binding::new(DUMMY_SP, Ident::gen("<name>"), Kind::example(),
      JType::example(), Some(Expr::example()))
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntailmentRel {
  pub left: Variable,
  pub right: Expr,
  pub strict: bool,
  pub span: Span
}

#[derive(Clone, Debug, Eq)]
pub struct Ident {
  pub value: String,
  pub span: Span
}

impl Ident {
  pub fn new(span: Span, value: String) -> Self {
    Ident {
      value: value,
      span: span
    }
  }

  pub fn gen(value: &str) -> Self {
    Ident::new(DUMMY_SP, String::from(value))
  }

  pub fn unwrap(&self) -> String {
    self.value.clone()
  }
}

impl Deref for Ident {
  type Target = String;
  fn deref(&self) -> &String {
    &self.value
  }
}

impl Display for Ident {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    fmt.write_str(&self.value)
  }
}

impl PartialEq for Ident {
  fn eq(&self, other: &Ident) -> bool {
    self.value == other.value
  }
}

impl Hash for Ident {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.value.hash(state);
  }
}

/// A variable path can be `x`, `m.x`, `m.m2.y`,... where `m` and `m2` must be checked to be module.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct VarPath {
  pub fragments: Vec<Ident>,
  pub span: Span
}

impl VarPath {
  pub fn new(span: Span, fragments: Vec<Ident>) -> Self {
    VarPath {
      fragments: fragments,
      span: span
    }
  }

  pub fn gen(value: &str) -> Self {
    VarPath::new(DUMMY_SP, vec![Ident::gen(value)])
  }

  pub fn empty() -> Self {
    VarPath::new(DUMMY_SP, vec![])
  }

  pub fn is_unary(&self) -> bool {
    self.fragments.len() == 1
  }

  pub fn name(&self) -> Ident {
    self.fragments.last().unwrap().clone()
  }

  pub fn target(&self) -> Ident {
    self.fragments.first().unwrap().clone()
  }
}

impl Display for VarPath {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let mut i = 0;
    while i < self.fragments.len() - 1 {
      fmt.write_fmt(format_args!("{}.", self.fragments[i]))?;
      i += 1;
    }
    fmt.write_str(self.fragments[i].as_str())
  }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Permission {
  Read,
  ReadWrite
}

#[derive(Clone, Debug, Eq)]
pub struct Variable {
  pub path: VarPath,
  /// This UID refers to the last variable of the path. In `m.x.y`, it refers to `y` and in `x`, it refers to `x`. This UID is used to retrieve global information about this variable through `Context`.
  pub uid: usize,
  pub past: usize,
  pub perm: Permission,
  pub span: Span
}

impl Variable {
  pub fn new(span: Span, path: VarPath, past: usize) -> Self {
    Variable {
      path: path,
      uid: 0,
      past: past,
      perm: Permission::ReadWrite,
      span: span
    }
  }

  pub fn simple(span: Span, name: Ident) -> Self {
    Self::present(span, VarPath::new(span, vec![name]))
  }

  pub fn present(span: Span, path: VarPath) -> Self {
    Self::new(span, path, 0)
  }

  pub fn name(&self) -> Ident {
    self.path.name()
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Self::simple(DUMMY_SP, Ident::gen("x"))
  }
}

impl PartialEq for Variable {
  fn eq(&self, other: &Variable) -> bool {
    assert!(self.uid > 0 && other.uid > 0,
      "Cannot compare variable before the UID is computed.");
    self.uid == other.uid
  }
}

impl Hash for Variable {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.uid.hash(state);
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
  /// A spacetime variable, either as a module's field or local to a process.
  Spacetime(Spacetime),
  /// A module variable: it is a Bonsai module, i.e. the product of heterogeneous spacetime variables.
  Product,
  /// A variable from the host language.
  Host
}

impl Kind {
  pub fn is_transient(self) -> bool {
    use self::Kind::*;
    match self {
      Spacetime(sp) => sp.is_transient(),
      _ => false
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Kind::Spacetime(Spacetime::example())
  }
}

/// The spacetime of a variable describes how it evolves in each instant. For `WorldLine` and `SingleSpace` we can additional set a boolean to `true` if the variable is transient (i.e. its value is re-initialized to bottom between each instant). The `Product` variant is used for records where variables have fields with various spacetime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spacetime {
  WorldLine(bool),
  SingleSpace(bool),
  SingleTime,
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
  NewInstance(JType, Vec<Expr>),
  CallChain(MethodCallChain),
  Boolean(bool),
  Number(u64),
  StringLiteral(String),
  Var(Variable),
  Bottom(JType)
}

impl ExprKind {
  #[allow(dead_code)]
  pub fn example() -> Self {
    ExprKind::Var(Variable::simple(DUMMY_SP, Ident::gen("<expr>")))
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodCallChain {
  pub calls: Vec<MethodCall>,
  pub span: Span
}

impl MethodCallChain {
  pub fn new(span: Span, calls: Vec<MethodCall>) -> Self {
    MethodCallChain {
      calls: calls,
      span: span
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodCall {
  pub target: VarPath,
  pub method: Ident,
  pub args: Vec<Expr>,
  pub span: Span
}

impl MethodCall {
  fn new(span: Span, target: VarPath, method: Ident, args: Vec<Expr>) -> Self {
    MethodCall {
      target: target,
      method: method,
      args: args,
      span: span,
    }
  }

  pub fn call_on_var(span: Span, target: VarPath, method: Ident, args: Vec<Expr>) -> Self {
    MethodCall::new(span, target, method, args)
  }

  pub fn call_on_this(span: Span, method: Ident, args: Vec<Expr>) -> Self {
    MethodCall::new(span, VarPath::gen("this"), method, args)
  }

  /// The target of the method is part of the `MethodCallChain` structure.
  pub fn call_fragment(span: Span, method: Ident, args: Vec<Expr>) -> Self {
    MethodCall::new(span, VarPath::empty(), method, args)
  }
}

/// Java fully qualified name (FQN).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FQN {
  pub names: Vec<Ident>,
  pub span: Span
}

impl FQN {
  pub fn new(span: Span, names: Vec<Ident>) -> Self {
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
  pub fqn: FQN,
  pub import_all: bool,
  pub span: Span
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
  pub class_name: Ident,
  pub interfaces: Vec<JType>,
  pub java_methods: Vec<JMethod>,
  pub java_constructors: Vec<JConstructor>,
}

impl JClass {
  pub fn new(header: String, package: FQN, imports: Vec<JImport>,
    class_name: Ident, interfaces: Vec<JType>) -> Self
  {
    JClass {
      header: header,
      package: package,
      imports: imports,
      class_name: class_name,
      interfaces: interfaces,
      java_methods: vec![],
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
  pub name: Ident,
  pub parameters: JParameters,
  pub body: JavaBlock,
  pub span: Span
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JConstructor {
  pub visibility: JVisibility,
  pub name: Ident,
  pub parameters: JParameters,
  pub body: JavaBlock,
  pub span: Span
}

pub type JavaBlock = String;
pub type JParameters = String;

#[derive(Clone, Debug, Eq)]
pub struct JType {
  pub name: Ident,
  pub generics: Vec<JType>,
  pub is_array: bool,
  pub span: Span
}

impl JType {
  pub fn simple(span: Span, name: Ident) -> Self {
    JType {
      name: name,
      generics: vec![],
      is_array: false,
      span: span
    }
  }

  pub fn example() -> Self {
    JType::simple(DUMMY_SP, Ident::gen("<Java type>"))
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
