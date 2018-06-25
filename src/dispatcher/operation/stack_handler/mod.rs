use std::sync::Mutex;

pub struct StackHandler {
    stk: Vec<i32>,
}

impl StackHandler {
    pub fn new() -> StackHandler {
        StackHandler{
            stk: Vec::new(),
        }
    }
    pub fn peek(&self) -> i32 {
        self.stk[self.stk.len() - 1]
    }
    pub fn pop(&mut self, i: usize) -> i32 {
        let mut diff = 0;
        for _ in 0..i {
            diff += self.stk.pop().expect("Popping from empty stack");
        }
        diff
    }
    pub fn push(&mut self, el: i32) {
        self.stk.push(el);
    }
    pub fn debug(&self) {
        println!("Stack size: {}", self.stk.len());
        println!("Stack: {:?}", self.stk);
    }
}

lazy_static!{
    pub static ref STACK_HOLDER: Mutex<StackHandler> = Mutex::new(StackHandler::new());
}