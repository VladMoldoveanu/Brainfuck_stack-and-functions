//! Contains the functions and a structure to hold them

use dispatcher::operation::base_operation::Operation;
use dispatcher::operation::array_handler::ArrayHandler;
use std::collections::VecDeque;

struct Function {
    ops: Vec<Operation>
}

impl Function {
    ///Execute the operations of a function
    pub fn execute(&self, ah: &mut ArrayHandler, fh: &FunctionHolder) {
        for op in (&self.ops).iter() {
            op.execute(ah, fh);
        }
    }
    ///Executes the operations of a function on a separate array
    pub fn execute_separate(&self, ah: &mut ArrayHandler, args: usize, fh: &FunctionHolder) {
        let mut ah2 = ArrayHandler::new((args + 1) * 2, false);
        ah2.set_more(ah, args);
        for op in (&self.ops).iter() {
            op.execute(&mut ah2, fh);
        }
    }
}

/// The structure which holds the functions
pub struct FunctionHolder {
    funs: Vec<Function>,
    temp: VecDeque<Function>
}

impl FunctionHolder {
    /// Creates a new empty function holder
    pub fn new() -> FunctionHolder {
        FunctionHolder {
            funs: Vec::new(),
            temp: VecDeque::new(),
        }
    }
    /// Ads a new temp function
    ///
    /// The functions are kept until they are pushed with the rest of them or discarded
    pub fn add_temp(&mut self, ops: Vec<Operation>) {
        self.temp.push_back(Function{ops});
    }
    /// Pushes the temp functions to the rest of them
    ///
    /// Usually called after a successful compile.
    pub fn push_funs(&mut self) {
        while !self.temp.is_empty() {
            self.funs.push(self.temp.pop_front().unwrap());
        }
    }
    /// Execute the function with the given number
    pub fn execute(&self, ah: &mut ArrayHandler) {
        let n = ah.get() as usize;
        assert!(n < self.funs.len());
        self.funs[n].execute(ah, self);
    }
    /// Execute the function with the given number on a separate array
    pub fn execute_separate(&self, ah: &mut ArrayHandler, args: usize) {
        let n = ah.get() as usize;
        assert!(n < self.funs.len());
        self.funs[n].execute_separate(ah, args, self);
    }
    /// Returns the number of stable function (without the temps)
    pub fn no_functions(&self) -> usize {
        self.funs.len()
    }
    /// Returns the current number of functions (stable + temp)
    pub fn curr_funs(&self) -> i32 {
        return (self.funs.len() + self.temp.len()) as i32
    }
    /// Discard the temp functions
    pub fn discard_funs(&mut self) {
        self.temp.truncate(0);
    }
}
