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
use driver::config::MainMethod;
use std::fmt::{Display, Error, Formatter};
use std::cmp::{Ordering, PartialEq};
use std::ops::Deref;
use std::hash::{Hash, Hasher};
pub use syntex_pos::Span;
pub use syntex_syntax::codemap::{mk_sp, DUMMY_SP};
pub use syntex_errors::Level;
pub use regex::Regex;
pub use trilean::SKleene;

#[derive(Clone, Debug)]
pub enum TestAnnotation {
  Compiler(CompilerTest),
  Execution(ExecutionTest)
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompilerTest {
  pub level: Level,
  pub code: String,
  pub line: usize,
  pub column: usize
}

impl Eq for CompilerTest { }

impl PartialOrd for CompilerTest {
  fn partial_cmp(&self, other: &CompilerTest) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for CompilerTest {
  fn cmp(&self, other: &CompilerTest) -> Ordering {
    self.line.cmp(&other.line)
      .then(self.column.cmp(&other.column))
      .then(self.code.cmp(&other.code))
  }
}

impl CompilerTest {
  pub fn new(level: String, code: String, line: usize, column: usize) -> Self {
    let level = CompilerTest::from_string_level(level);
    CompilerTest {
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

impl Display for CompilerTest
{
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    fmt.write_fmt(format_args!("{}:{}:{}:{}", self.level,
      self.code, self.line, self.column))
  }
}

/// Given a test specification `#[run(process, regex)]`, execute the process `process` and check its printed output with `regex`.
#[derive(Clone, Debug)]
pub struct ExecutionTest {
  pub process: MainMethod,
  pub output_regex: Regex,
  pub filter_debug: bool
}

impl ExecutionTest {
  pub fn new(class_name: String, process_name: String, regex: String, filter_debug: bool) -> Self {
    let output_regex = Regex::new(&regex).unwrap();
    ExecutionTest {
      process: MainMethod::new(class_name, process_name), output_regex, filter_debug }
  }
}

#[derive(Clone, Debug)]
pub struct Crate<Host> {
  pub modules: Vec<Module<Host>>,
}

impl<Host> Crate<Host> {
  pub fn new() -> Self {
    Crate {
      modules: vec![],
    }
  }
}

impl Crate<JClass> {
  pub fn find_mod_by_name(&self, name: &Ident) -> Option<Module<JClass>> {
    self.modules.iter()
      .find(|m| &m.mod_name() == name).cloned()
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
  pub fn ref_fields(&self) -> Vec<ModuleField> {
    self.fields.iter()
      .filter(|a| a.is_ref.is_some())
      .cloned()
      .collect()
  }

  pub fn find_field_by_name(&self, name: &Ident) -> Option<ModuleField> {
    self.fields.iter()
      .find(|f| &f.binding.name == name).cloned()
  }

  pub fn find_process_by_name(&self, name: &Ident) -> Option<Process> {
    self.processes.iter()
      .find(|p| &p.name == name).cloned()
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

  pub fn mod_name(&self) -> Ident {
    self.host.class_name.clone()
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleField {
  pub visibility: JVisibility,
  pub binding: Binding,
  pub is_ref: Option<Span>,
  pub is_static: bool,
  pub is_final: bool,
  pub span: Span
}

impl ModuleField {
  fn new(span: Span, visibility: Option<JVisibility>,
    binding: Binding, is_ref: Option<Span>, is_static: bool, is_final: bool) -> Self
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
    binding: Binding, is_ref: Option<Span>) -> Self
  {
    let is_final = binding.kind != Kind::Spacetime(Spacetime::SingleTime);
    ModuleField::new(span, visibility, binding, is_ref, false, is_final)
  }

  pub fn java_field(span: Span, visibility: Option<JVisibility>,
    binding: Binding, is_static: bool, is_final: bool) -> Self
  {
    ModuleField::new(span, visibility, binding, None, is_static, is_final)
  }
}

#[derive(Clone, Debug)]
pub struct Program {
  pub header: String,
  pub tests: Vec<TestAnnotation>,
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
  OrPar(Vec<Stmt>),  // par p <> q end
  AndPar(Vec<Stmt>), // par p || q end
  Space(Box<Stmt>),
  Prune,
  Let(LetStmt),
  When(Expr, Box<Stmt>, Box<Stmt>),
  Suspend(SuspendStmt),
  Abort(Expr, Box<Stmt>),
  Tell(Variable, Expr),
  DelayStmt(Delay),
  Loop(Box<Stmt>),
  ProcCall(Option<Variable>, Ident, Vec<Variable>),
  ExprStmt(Expr),
  QFUniverse(Box<Stmt>), // Queue-free universe
  Universe(Variable, Box<Stmt>), // Queue-free universe
  Nothing,
}

impl StmtKind {
  #[allow(dead_code)]
  pub fn example() -> Self {
    StmtKind::Tell(Variable::example(), Expr::example())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Delay {
  pub kind: DelayKind,
  pub state_num: usize
}

impl Delay {
  pub fn new(kind: DelayKind) -> Self {
    Delay { kind, state_num: 0}
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DelayKind {
  Pause,
  PauseUp,
  Stop
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuspendStmt {
  pub condition: Expr,
  pub body: Box<Stmt>,
  pub state_num: usize,
}

impl SuspendStmt {
  pub fn new(condition: Expr, body: Box<Stmt>) -> Self {
    SuspendStmt { condition, body, state_num: 0 }
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

  pub fn kind(&self) -> Kind {
    self.binding.kind
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

  pub fn is_module(&self) -> bool {
    self.kind == Kind::Product
  }

  pub fn is_host(&self) -> bool {
    self.kind == Kind::Host
  }

  pub fn is_single_time(&self) -> bool {
    self.kind == Kind::Spacetime(Spacetime::SingleTime)
  }

  pub fn to_field_var(self) -> Variable {
    let mut path = VarPath::new(self.span, vec![self.name]);
    path.uids[0] = self.uid;
    Variable { op_no: 0, path, with_this: true, past: 0, permission: None, span: self.span }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Binding::new(DUMMY_SP, Ident::gen("<name>"), Kind::example(),
      JType::example(), Some(Expr::example()))
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntailmentKind {
  Entailment,
  StrictEntailment,
  Equality
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntailmentRel {
  pub left: Expr,
  pub right: Expr,
  pub op: EntailmentKind
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
#[derive(Clone, Debug, Eq)]
pub struct VarPath {
  pub fragments: Vec<Ident>,
  /// These UIDs match the fragments of the path.
  /// These UIDs are used to retrieve global information about variables through `Context`.
  pub uids: Vec<usize>,
  pub span: Span
}

impl VarPath {
  pub fn new(span: Span, fragments: Vec<Ident>) -> Self {
    let len = fragments.len();
    VarPath {
      fragments: fragments,
      uids: (0..len).map(|_| 0).collect(),
      span: span
    }
  }

  pub fn gen(value: &str) -> Self {
    VarPath::new(DUMMY_SP, vec![Ident::gen(value)])
  }

  pub fn len(&self) -> usize {
    self.fragments.len()
  }

  pub fn first(&self) -> Ident {
    self.fragments[0].clone()
  }

  pub fn last(&self) -> Ident {
    self.fragments.last().unwrap().clone()
  }

  pub fn first_uid(&self) -> usize {
    *self.uids.first().unwrap()
  }

  pub fn last_uid(&self) -> usize {
    *self.uids.last().unwrap()
  }

  pub fn extract_this(&mut self) -> bool {
    let first = format!("{}", self.first());
    if first == "this" {
      self.fragments.remove(0);
      self.uids.remove(0);
      assert!(self.len() > 0,
        "Variable path with a unique `this`. A variable prefixed with `this` must refer to a field.");
      true
    }
    else { false }
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

impl PartialEq for VarPath {
  fn eq(&self, other: &VarPath) -> bool {
    if self.uids.len() == other.uids.len() {
      for i in 0..self.uids.len() {
        if self.uids[i] != other.uids[i] {
          return false;
        }
      }
      true
    }
    else { false }
  }
}

impl Hash for VarPath {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.uids.hash(state);
  }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Permission {
  Read,
  ReadWrite,
  Write
}

impl PartialOrd for Permission {
  fn partial_cmp(&self, other: &Permission) -> Option<Ordering> {
    if self == other { Some(Ordering::Equal) }
    else {
      match *self {
        Permission::Read => Some(Ordering::Less),
        Permission::Write => Some(Ordering::Greater),
        Permission::ReadWrite => Some(other.partial_cmp(self).unwrap().reverse())
      }
    }
  }
}

impl Display for Permission {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    match *self {
      Permission::Read => fmt.write_str("read"),
      Permission::Write => fmt.write_str("write"),
      Permission::ReadWrite => fmt.write_str("readwrite"),
    }
  }
}

#[derive(Clone, Debug, Eq)]
pub struct Variable {
  pub op_no: usize, // This field represents the operation number computed and used during the causality analysis.
  pub path: VarPath,
  pub with_this: bool, // If the variable path starts with `this` (e.g. `this.x`), then we must look it up in the fields hashmap.
  pub past: usize,
  pub permission: Option<Permission>,
  pub span: Span
}

impl Variable {
  fn new(span: Span, mut path: VarPath, past: usize,
    permission: Option<Permission>) -> Self
  {
    let with_this = Variable::extract_this(&mut path);
    Variable {
      op_no: 0,
      path: path,
      with_this: with_this,
      past: past,
      permission: permission,
      span: span
    }
  }

  fn extract_this(path: &mut VarPath) -> bool {
    path.extract_this()
  }

  pub fn stream(span: Span, path: VarPath, past: usize) -> Self {
    Self::new(span, path, past, Some(Permission::Read))
  }

  pub fn access(span: Span, path: VarPath, permission: Option<Permission>) -> Self {
    Self::new(span, path, 0, permission)
  }

  pub fn first(&self) -> Ident {
    self.path.first()
  }

  pub fn first_uid(&self) -> usize {
    self.path.first_uid()
  }

  pub fn last(&self) -> Ident {
    self.path.last()
  }

  pub fn last_uid(&self) -> usize {
    self.path.last_uid()
  }

  pub fn len(&self) -> usize {
    self.path.len()
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Self::access(DUMMY_SP, VarPath::gen("x"), Some(Permission::ReadWrite))
  }
}

impl PartialEq for Variable {
  fn eq(&self, other: &Variable) -> bool {
    self.path == other.path && self.past == other.past
  }
}

impl Hash for Variable {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.path.hash(state);
  }
}

impl Display for Variable {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    match self.permission {
      Some(p) => { fmt.write_fmt(format_args!("{}", p))?; }
      None => (),
    };
    fmt.write_fmt(format_args!("({}) {}({})",
      self.op_no, self.path, self.last_uid()))
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
  #[allow(dead_code)]
  pub fn example() -> Self {
    Kind::Spacetime(Spacetime::example())
  }
}

impl Display for Kind {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    match self {
      &Kind::Spacetime(sp) => sp.fmt(fmt),
      &Kind::Product => fmt.write_str("module"),
      &Kind::Host => fmt.write_str("Java")
    }
  }
}

/// The spacetime of a variable describes how it evolves in each instant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spacetime {
  WorldLine,
  SingleSpace,
  SingleTime,
}

impl Spacetime {
  #[allow(dead_code)]
  pub fn example() -> Self {
    Spacetime::SingleSpace
  }
}

impl Display for Spacetime {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    let st = match self {
      &Spacetime::WorldLine => "world_line",
      &Spacetime::SingleSpace => "single_space",
      &Spacetime::SingleTime => "single_time"
    };
    fmt.write_str(st)
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

  pub fn is_var(&self) -> bool {
    match &self.node {
      &ExprKind::Var(_) => true,
      _ => false
    }
  }

  #[allow(dead_code)]
  pub fn example() -> Self {
    Expr::new(DUMMY_SP, ExprKind::example())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExprKind {
  // Host expressions
  Number(u64),
  StringLiteral(String),
  NewInstance(NewObjectInstance),
  Call(MethodCall),
  // Bonsai expressions
  Var(Variable),
  Bottom,
  Top,
  // Trilean
  Trilean(SKleene),
  Or(Box<Expr>, Box<Expr>),
  And(Box<Expr>, Box<Expr>),
  Not(Box<Expr>),
  Entailment(Box<EntailmentRel>)
}

impl ExprKind {
  #[allow(dead_code)]
  pub fn example() -> Self {
    ExprKind::Var(Variable::access(DUMMY_SP, VarPath::gen("<expr>"), Some(Permission::Read)))
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewObjectInstance {
  pub span: Span,
  pub ty: JType,
  pub args: Vec<Expr>
}

impl NewObjectInstance {
  pub fn new(span: Span, ty: JType, args: Vec<Expr>) -> Self {
    NewObjectInstance {span, ty, args}
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodCall {
  /// If `None` the method called is supposed `static` (we forbid method call on `this`, see issue#3).
  pub target: Option<Variable>,
  pub method: Ident,
  pub args: Vec<Expr>,
  pub span: Span
}

impl MethodCall {
  pub fn new(span: Span, target: Option<Variable>, method: Ident, args: Vec<Expr>) -> Self {
    MethodCall { span, target, method, args }
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

  pub fn is_empty(&self) -> bool {
    self.names.is_empty()
  }
}

impl Display for FQN {
  fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
    if !self.is_empty() {
      let mut i = 0;
      while i < self.names.len() - 1 {
        fmt.write_fmt(format_args!("{}.", self.names[i]))?;
        i += 1;
      }
      fmt.write_str(self.names[i].as_str())?;
    }
    Ok(())
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JImport {
  pub fqn: FQN,
  pub import_all: bool, // `import A.B.*;`
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

  pub fn is_imported(&self, class_name: &Ident) -> bool {
    for import in &self.imports {
      if !import.import_all && import.fqn.names.last().unwrap() == class_name {
        return true;
      }
    }
    false
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
pub type JParameters = Vec<JParameter>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JParameter {
  pub name: Ident,
  pub ty: JType,
  pub span: Span
}

impl JParameter {
  pub fn new(span: Span, ty: JType, name: Ident) -> Self {
    JParameter {
      name: name,
      ty: ty,
      span: span
    }
  }
}

impl Display for JParameter
{
  fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
    formatter.write_fmt(format_args!("{} ", self.ty))?;
    formatter.write_fmt(format_args!("{}", self.name))
  }
}


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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
