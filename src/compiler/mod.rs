use dispatcher::dispatch;
use dispatcher::Dispatcher::{Op, Empty, Fun, Error};
use dispatcher::operation::base_operation::Operation;
use dispatcher::operation::array_handler::ArrayHandler;
use dispatcher::operation::function::FunctionHolder;
use std::collections::VecDeque;
use reader::Reader;
use std::time::SystemTime;
use optimiser::optimise_code;

enum CompileError {
    Ok,
    Error(String)
}

pub struct Compiler {
    ops: VecDeque<Operation>,
    temp: VecDeque<Operation>,
    ah: ArrayHandler,
    fh: FunctionHolder,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            ops: VecDeque::new(),
            temp: VecDeque::new(),
            ah: ArrayHandler::new(1024, true),
            fh: FunctionHolder::new(),
        }
    }
    pub fn compile_file(&mut self, fname: String) {
        println!("Compiling file '{}'", fname);
        let now = SystemTime::now();
        let mut reader = Reader::from_file(fname.clone());
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