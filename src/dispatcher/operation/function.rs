use dispatcher::operation::base_operation::Operation;
use dispatcher::operation::array_handler::ArrayHandler;
use std::collections::VecDeque;

struct Function {
    ops: Vec<Operation>
}

impl Function {
    pub fn execute(&self, ah: &mut ArrayHandler, fh: &FunctionHolder) {
        for op in (&self.ops).iter() {
            op.execute(ah, fh);
        }
    }
    pub fn execute_separate(&self, ah: &mut ArrayHandler, args: usize, fh: &FunctionHolder) {
        let mut ah2 = ArrayHandler::new((args + 1) * 2, false);
        ah2.set_more(ah, args);
        for op in (&self.ops).iter() {
            op.execute(&mut ah2, fh);
        }
    }
}

pub struct FunctionHolder {
    funs: Vec<Function>,
    temp: VecDeque<Function>
}

impl FunctionHolder {
    pub fn new() -> FunctionHolder {
        FunctionHolder {
            funs: Vec::new(),
            temp: VecDeque::new(),
        }
    }
    pub fn add_temp(&mut self, ops: Vec<Operation>) {
        self.temp.push_back(Function{ops});
    }
    pub fn push_funs(&mut self) {
        while !self.temp.is_empty() {
            self.funs.push(self.temp.pop_front().unwrap());
        }
    }
    pub fn execute(&self, ah: &mut ArrayHandler) {
        let n = ah.get() as usize;
        assert!(n < self.funs.len());
        self.funs[n].execute(ah, self);
    }
    pub fn execute_separate(&self, ah: &mut ArrayHandler, args: usize) {
        let n = ah.get() as usize;
        assert!(n < self.funs.len());
        self.funs[n].execute_separate(ah, args, self);
    }
    pub fn no_functions(&self) -> usize {
        self.funs.len()
    }
    pub fn curr_funs(&self) -> i32 {
        return (self.funs.len() + self.temp.len()) as i32
    }
    pub fn discard_funs(&mut self) {
        self.temp.truncate(0);
    }
}
