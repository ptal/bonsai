// Copyright 2017 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use context::*;
use session::*;
use driver::config::MainMethod;
use back::code_formatter::*;
use back::compiler::expression::*;
use back::compiler::statement::*;

pub fn compile_module(env: Env<Context>, module: JModule) -> Env<(Context, String)> {
  env.and_next(|session, context| {
    let mod_name = module.mod_name();
    let code = ModuleCompiler::new(&session, &context, mod_name).compile(module);
    Env::new(session, code.map(|code| (context, code)))
  })
}

struct ModuleCompiler<'a> {
  session: &'a Session,
  context: &'a Context,
  mod_name: Ident,
  fmt: CodeFormatter
}

impl<'a> ModuleCompiler<'a>
{
  fn new(session: &'a Session, context: &'a Context, mod_name: Ident) -> Self {
    ModuleCompiler {
      session, context, mod_name,
      fmt: CodeFormatter::new()
    }
  }

  fn compile(mut self, module: JModule) -> Partial<String> {
    self.header(&module.host);
    self.class(module);
    Partial::Value(self.fmt.unwrap())
  }

  fn header(&mut self, jclass: &JClass) {
    self.fmt.push_block(jclass.header.clone());
    self.fmt.push_line(&format!("package {};", jclass.package));
    for import in &jclass.imports {
      self.fmt.push_line(&format!("import {};", import));
    }
    self.runtime_imports();
  }

  fn runtime_imports(&mut self) {
    self.fmt.push_line("import bonsai.runtime.core.*;");
    self.fmt.push_line("import bonsai.runtime.lattices.*;");
    self.fmt.push_line("import bonsai.runtime.synchronous.*;");
    self.fmt.push_line("import bonsai.runtime.synchronous.env.*;");
    self.fmt.push_line("import bonsai.runtime.synchronous.statements.*;");
    self.fmt.push_line("import bonsai.runtime.synchronous.expressions.*;");
    self.fmt.push_line("import bonsai.runtime.synchronous.interfaces.*;");
  }

  fn class(&mut self, module: JModule) {
    self.class_decl(&module.host);
    self.fmt.open_block();
    for field in module.fields.clone() {
      self.field(field);
    }
    self.runtime_boilerplate(&module);
    self.main_method(module.host.class_name);
    for process in module.processes {
      self.process(process);
    }
    for method in module.host.java_methods {
      self.java_method(method);
    }
    for constructor in module.host.java_constructors {
      self.java_constructor(constructor);
    }
    self.fmt.close_block();
  }

  fn runtime_boilerplate(&mut self, module: &JModule) {
    self.runtime_object_uid(module);
    // self.runtime_init_method(module);
    // self.runtime_destroy_method(module);
  }

  fn class_decl(&mut self, jclass: &JClass) {
    self.fmt.push(&format!("public class {}", jclass.class_name));
    self.interfaces(jclass.interfaces.clone());
    self.fmt.newline();
  }

  fn interfaces(&mut self, interfaces: Vec<JType>) {
    if !interfaces.is_empty() {
      self.fmt.push(" implements ");
      let len = interfaces.len();
      for (i, interface) in interfaces.into_iter().enumerate() {
        self.fmt.push(&format!("{}", interface));
        if i != len - 1 {
          self.fmt.push(", ");
        }
      }
    }
  }

  fn main_method(&mut self, class_name: Ident) {
    let main_expr = match self.session.config().main_method.clone() {
      Some(MainMethod { ref class, ref method }) if *class == *class_name => {
        Some(format!("new {}().{}()", class, method))
      },
      _ => None
    };
    if let Some(main_expr) = main_expr {
      self.fmt.push_line("public static void main(String[] args)");
      self.fmt.open_block();
      self.fmt.push_block(format!("\
        Statement program = {};\n\
        SpaceMachine machine = new SpaceMachine(program,{});\n\
        machine.execute();", main_expr, self.session.config().debug));
      self.fmt.close_block();
      self.fmt.newline();
    }
  }

  fn java_method(&mut self, method: JMethod) {
    let header: String = vec![
      format!("{} ", method.visibility),
      Self::string_from_static(method.is_static),
      format!("{} ", method.return_ty),
      method.name.unwrap()
    ].iter().flat_map(|x| x.chars()).collect();
    self.fmt.push(&header);
    self.params_list(method.parameters);
    self.fmt.push_java_block(method.body);
  }

  fn java_constructor(&mut self, constructor: JConstructor) {
    let header: String = vec![
      format!("{} ", constructor.visibility),
      constructor.name.unwrap()
    ].iter().flat_map(|x| x.chars()).collect();
    self.fmt.push(&header);
    self.params_list(constructor.parameters);
    self.fmt.push_java_block(constructor.body);
  }

  fn params_list(&mut self, parameters: JParameters) {
    self.fmt.push("(");
    let len = parameters.len();
    for (i, param) in parameters.into_iter().enumerate() {
      self.fmt.push(&format!("{}", param));
      if i != len - 1 {
        self.fmt.push(", ");
      }
    }
    self.fmt.push(")");
  }

  fn field(&mut self, field: ModuleField) {
    let code: String = vec![
      Self::string_from_final(field.is_final),
      format!("{} ", field.visibility),
      Self::string_from_static(field.is_static),
      format!("{} ", field.binding.ty),
      field.binding.name.unwrap()
    ].iter().flat_map(|x| x.chars()).collect();
    self.fmt.push(&code);
    if let Some(expr) = field.binding.expr {
      self.fmt.push(" = ");
      if expr.node == ExprKind::Bottom {
        self.fmt.push(&format!("new {}()", field.binding.ty.name));
      }
      else {
        compile_expression(self.session, self.context, &mut self.fmt, expr);
      }
    }
    self.fmt.terminate_line(";");
  }

  // fn runtime_init_method(&mut self, module: &JModule) {
  //   self.fmt.push_line("public void __init(Layer senv)");
  //   self.fmt.open_block();
  //   self.fmt.push_line("__num_instances++;");
  //   self.fmt.push_line("__object_instance = __num_instances;");
  //   for field in module.fields.clone() {
  //     let binding = field.binding;
  //     if binding.is_module() {
  //       self.fmt.push_line(&format!("{}.__init(senv);", binding.name));
  //     }
  //     else {
  //       self.fmt.push("senv.enterScope(");
  //       // self.binding(binding, true, "__uid");
  //       self.fmt.terminate_line(");");
  //     }
  //   }
  //   self.fmt.close_block();
  // }

  // fn runtime_destroy_method(&mut self, module: &JModule) {
  //   self.fmt.push_line("public void __destroy(Layer senv)");
  //   self.fmt.open_block();
  //   for field in module.fields.clone() {
  //     let binding = field.binding;
  //     if binding.is_module() {
  //       self.fmt.push_line(&format!("{}.__destroy(senv);", binding.name));
  //     }
  //     else {
  //       self.fmt.push_line(&format!("senv.exitScope(__uid(\"{}\"));", binding.name));
  //     }
  //   }
  //   self.fmt.close_block();
  // }

  fn runtime_object_uid(&mut self, module: &JModule) {
    self.fmt.push_line("private static int __num_instances = -1;");
    self.fmt.push_line("private int __object_instance;");
    self.fmt.push_line("public String __uid(String var)");
    self.fmt.open_block();
    // return "[package]." + "[classname]." + __object_instance + "." + var;
    self.fmt.push_line(&format!(
      "return \"{}.\" + \"{}.\" + __object_instance + \".\" + var;",
      module.host.package, module.host.class_name));
    self.fmt.close_block();
  }

  fn proc_uid(&mut self, process: &Process, proc_instance: String) {
    self.fmt.push_line(&format!("{}++;", proc_instance));
    // Avoid the capture of the static variable `__proc_{}_instance` in the closure `__proc_uid`: we need its current value.
    self.fmt.push_line(&format!("int __proc_instance = {};", proc_instance));
    self.fmt.push_line("java.util.function.Function<String, String> __proc_uid = ");
    self.fmt.push_line(&format!(
      "  (var) -> __uid(\"{}.\" + __proc_instance + \".\" + var);",
      process.name));
  }

  fn string_from_final(is_final: bool) -> String {
    if is_final {
      String::from("final ")
    }
    else { String::new() }
  }

  fn string_from_static(is_static: bool) -> String {
    if is_static {
      String::from("static ")
    }
    else { String::new() }
  }

  fn process(&mut self, process: Process) {
    let proc_instance = format!("__proc_{}_instance", process.name);
    self.fmt.push_line(&format!("static int {} = -1;", proc_instance));
    self.fmt.push(&format!(
      "{} Statement {}", process.visibility, process.name));
    self.params_list(process.params.clone());
    self.fmt.open_block();
    self.proc_uid(&process, proc_instance);
    self.fmt.push_line("return");
    self.fmt.indent();
    compile_statement(self.session, self.context, &mut self.fmt, self.mod_name.clone(), process.body);
    self.fmt.unindent();
    self.fmt.terminate_line(";");
    self.fmt.close_block();
    self.fmt.newline();
  }
}
