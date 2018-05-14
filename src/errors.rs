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

#![allow(non_snake_case)]

register_long_diagnostics! {
E0001: r##"Unknown bonsai module."##,
E0002: r##"Duplicate field in a module."##,
E0003: r##"Duplicate local variable in a process."##,
E0004: r##"Duplicate process in a module."##,
E0005: r##"`ref` variable occurrence in field initialization."##,
E0006: r##"Undeclared variable (local to the module)."##,
E0007: r##"Undeclared process (local to the module)."##,
E0008: r##"Access to an unknown field (external to the current module)."##,
E0009: r##"Invocation of an unknown process (external to the current module)."##,
E0010: r##"Process call on a foreign object (from the host language)."##,
E0011: r##"`ref` field must not be initialized."##,
E0012: r##"Multiple constructor in a module with `ref` fields."##,
E0013: r##"Missing constructor in a module with `ref` fields."##,
E0014: r##"Missing parameter initializing a `ref` field in the constructor of the module."##,
E0015: r##"Mismatch between constructor parameter and `ref` field of the module."##,
E0016: r##"Writing on a `pre` variable. For example: `pre x <- e`."##,
E0017: r##"Illegal kind of a variable under a `pre` operator."##,
E0018: r##"Local variables of kind `module` with `ref` fields must be instantiated with the `new` operator."##,
E0019: r##"Local variables of kind `module` must always be initialized."##,
E0020: r##"Illegal kind of `ref` field (must be of the `spacetime` kind)."##,
E0021: r##"Field of kind `module` where the module has `ref` fields must not be initialized."##,
E0022: r##"`ref` argument when calling a module constructor must be a variable."##,
E0023: r##"`ref` argument must match the type and kind of the called constructor's parameters."##,
E0024: r##"Constructor's parameters list and instantiation list differ in size."##,
E0025: r##"Missing spacetime specifier for local host variable."##,
E0026: r##"Variable accessed with an illegal permission in the current context."##,
E0027: r##"Illegal host function call in a read only context (e.g. an entailment expression)."##,
E0028: r##"`loop` statement with an instantaneous body."##,
E0029: r##"`space` statement with a body that is not instantaneous."##,
E0030: r##"Recursive process calls are forbidden."##,
E0031: r##"Search statement (`space` and `prune`) in the process `p` of a `space p end` statement."##,
}
