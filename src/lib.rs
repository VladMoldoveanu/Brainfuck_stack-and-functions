//! Collects all separate modules and imports the extern crate `lazy_static`.
//! Imports only the functions/structures needed for running the program.
//!
//! # Examples
//!
//! All you need for starting the interpreter is creating a new compiler
//! with: `let mut compiler = Compiler::new();`
//!
//! and run the program: `run(&mut Compiler);`
//!
mod reader;

mod dispatcher;

mod compiler;
pub use compiler::Compiler;

mod cmd_loop;
pub use cmd_loop::run;

mod optimiser;

#[macro_use]
extern crate lazy_static;
