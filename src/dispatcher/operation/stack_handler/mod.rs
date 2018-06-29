//! Handles the global stack of the interpreter.

use std::sync::Mutex;

/// The structure which holds the stack
pub struct StackHandler {
    stk: Vec<i32>,
}

impl StackHandler {
    ///Creates a new stack
    pub fn new() -> StackHandler {
        StackHandler{
            stk: Vec::new(),
        }
    }
    /// Returns the top of the stack without popping it
    pub fn peek(&self) -> i32 {
        self.stk[self.stk.len() - 1]
    }
    /// Pops and returns the top of the stack
    pub fn pop(&mut self, i: usize) -> i32 {
        let mut diff = 0;
        for _ in 0..i {
            diff += self.stk.pop().expect("Popping from empty stack");
        }
        diff
    }
    /// Pushes an element to the stack
    pub fn push(&mut self, el: i32) {
        self.stk.push(el);
    }
    /// Prints the debug information
    pub fn debug(&self) {
        println!("Stack size: {}", self.stk.len());
        println!("Stack: {:?}", self.stk);
    }
    /// Returns the length of the stack
    pub fn len(&self) -> usize {
        self.stk.len()
    }
}

lazy_static!{
    /// The global stack used by the operations
    pub static ref STACK_HOLDER: Mutex<StackHandler> = Mutex::new(StackHandler::new());
}