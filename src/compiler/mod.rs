//! Provides an object able to compile and execute scripts.
//!
//! Can compile multiple files before running, although it is not
//! recommended due to the new features of the functions.
//!
//! Loading multiple files in the interpreter at the same time results in
//! each being compiled and instantly run after.

use dispatcher::*;
use std::collections::VecDeque;
use reader::Reader;
use std::time::SystemTime;
use optimiser::optimise_code;

enum CompileError {
    Ok,
    Error(String)
}
/// Main structure of the module, stores all the necessary data to run scripts
///
/// Each Compiler has its own set of functions, so creating multiple Compilers is not recommended
///
pub struct Compiler {
    ops: VecDeque<Operation>,
    temp: VecDeque<Operation>,
    ah: ArrayHandler,
    fh: FunctionHolder,
}

impl Compiler {
    /// Creates a new Compiler. The default array for executing operations goes
    /// from -512 to 511, but is dynamically sized.
    ///
    /// # Examples
    /// Standard usage:
    ///
    /// ```
    /// use Brainfuck::Compiler;
    ///
    /// let mut compiler = Compiler::new();
    /// //do stuff with the compiler
    /// ```
    pub fn new() -> Compiler {
        Compiler {
            ops: VecDeque::new(),
            temp: VecDeque::new(),
            ah: ArrayHandler::new(1024, true),
            fh: FunctionHolder::new(),
        }
    }
    /// Compiles a script file, optimising the operations and timing the process.
    ///
    /// # Examples
    /// ```
    /// use Brainfuck::Compiler;
    ///
    /// let mut compiler = Compiler::new();
    /// compiler.compile_file(String::from("loadtest"));
    /// // Now the operations from this file can be executed,
    /// // but this file contains only a function.
    /// ```
    /// # Errors
    ///
    /// If the file cannot be open/read, prints the error and does nothing.
    ///
    /// If the timer returns an error, prints it and finishes compiling.
    ///
    /// If the file contains any syntax errors, stops at the first error detected
    /// and prints it, without compiling anything.
    ///
    pub fn compile_file(&mut self, fname: String) {
        println!("Compiling file '{}'", fname);
        let now = SystemTime::now();
        let mut reader = match Reader::from_file(fname.clone()) {
            Ok(rdr) => rdr,
            Err(s) => {
                println!("{}",s);
                return;
            }
        };
        let res = loop {
            match dispatch(&mut reader, &mut self.fh) {
                Op(op) => self.temp.push_back(op),
                Fun => {}
                Empty => break CompileError::Ok,
                Error(s) => break CompileError::Error(s),
            }
        };
        match res {
            CompileError::Ok => {
                self.fh.push_funs();
                //println!("Before optimisation: {:?}", self.temp);
                self.temp = optimise_code(self.temp.clone());
                //println!("Optimised operations: {:?}", self.temp);
                while !self.temp.is_empty() {
                    self.ops.push_back(self.temp.pop_front().unwrap());
                }
                self.temp.shrink_to_fit();
                match now.elapsed() {
                    Ok(elapsed) => {
                        let in_ms = (elapsed.as_secs() * 1000) as f64 +
                            elapsed.subsec_nanos() as f64 / 1_000_000f64;
                        println!("File '{}' compiled, elapsed time: {}ms", fname, in_ms);
                    }
                    Err(e) => println!("Timer error: {:?}\nFile '{}' compiled successfully", e, fname),
                }
            }
            CompileError::Error(s) => {
                self.fh.discard_funs();
                self.temp.truncate(0);
                println!("{}\nFile {} not compiled", s, fname);
            }
        }
    }
    /// Takes a String and compiles the scripts contained by it
    ///
    /// # Examples
    /// ```
    /// use Brainfuck::Compiler;
    ///
    /// let mut compiler = Compiler::new();
    /// //Pushes Set(0) as the last operation to be executed;
    /// compiler.compile_string(String::from("[-]"));
    /// compiler.execute(false);
    /// ```
    /// # Errors
    ///
    /// If the string contains any syntax errors, prints the first error and
    /// doesn't compile anything.
    ///
    pub fn compile_string(&mut self, s: String) {
        let mut reader = Reader::from_string(s);
        let res = loop {
            match dispatch(&mut reader, &mut self.fh) {
                Op(op) => self.temp.push_back(op),
                Fun => {}   
                Empty => break CompileError::Ok,
                Error(s) => break CompileError::Error(s),
            }
        };
        match res {
            CompileError::Ok => {
                self.fh.push_funs();
                //println!("Before optimisation: {:?}", self.temp);
                self.temp = optimise_code(self.temp.clone());
                //println!("Optimised operations: {:?}", self.temp);
                while !self.temp.is_empty() {
                    self.ops.push_back(self.temp.pop_front().unwrap());
                }
                self.temp.shrink_to_fit();
            }
            CompileError::Error(s) => {
                self.fh.discard_funs();
                self.temp.truncate(0);
                println!("{}", s);
            }
        }
    }
    /// Runs the operations compiled since the last execute
    ///
    /// If `timed` is true, times the execution of the operations, printing
    /// on the screen at the end the time elapsed.
    ///
    /// # Examples
    /// ```
    /// use Brainfuck::Compiler;
    ///
    /// let mut compiler = Compiler::new();
    /// //Pushes Set(0) as the last operation to be executed;
    /// compiler.compile_string(String::from("[-]"));
    /// //Executes the Set(0) operation timed.
    /// compiler.execute(true);
    /// //Doesn't execute anything
    /// compiler.execute(false);
    /// ```
    ///
    /// # Errors
    ///
    /// If the timer has any errors, prints the error, but doesn't affect the operations in any way.
    ///
    pub fn execute(&mut self, timed: bool) {
        let now = SystemTime::now();
        for i in self.ops.iter() {
            i.execute(&mut self.ah, &mut self.fh);
        }
        if timed {
            match now.elapsed() {
                Ok(elapsed) => {
                    let in_ms = (elapsed.as_secs() * 1000) as f64 +
                        elapsed.subsec_nanos() as f64 / (1_000_000 as f64);
                    println!("Time elapsed: {}ms", in_ms);
                }
                Err(e) => println!("Unexpected timer error: {:?}", e),
            }
        }
        self.ops = VecDeque::new();
    }
}