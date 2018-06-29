//! The operations that are executed

use dispatcher::operation::*;
use dispatcher::operation::stack_handler::STACK_HOLDER;

/// All the types of operations
#[derive(Clone, Debug)]
pub enum Operation {
    /// Add x to the current position
    Add(i32),
    /// Move the pointer by x spaces
    Move(i32),
    /// Add the current value to multiple locations multiplied
    /// by a specified value
    MoveTo(Vec<(i32, i32)>),
    /// Equivalent to Loop(Move x)
    SkipMove(i32),
    /// Set the value at the current position to x
    Set(i32),
    /// Read next character
    Read,
    /// Write the character at the current position
    Write,
    /// While operation
    While(Vec<Operation>),
    /// Insert the total number of functions at the current position
    InsFuns,
    /// Calls the function with the number specified
    /// by the current position on the current array
    CallFun,
    /// Calls the function with the number specified
    /// by the current position on a separate array
    /// with x arguments copied
    CallFSep(usize),
    /// Prints debug information
    Debug,
    /// Inserts the number at the top of the stack
    /// at the current position
    PeekStack,
    /// Pops x elements from the stack, inserting the last one
    /// at the current position
    PopStack(usize),
    /// Push the number at the current position to the stack
    PushStack,
    /// Inserts the length of the stack at the current position
    StackLen,
    /// An operation that does nothing and is reduced during optimisation
    EmptyOp,
}

impl Operation {
    /// Maps each Operation to its specified behaviour
    pub fn execute(& self, ah: &mut ArrayHandler, fun_holder: &FunctionHolder) {
        match self {
            &Add(i) => ah.add(i),
            &Move(i) => ah.move_r(i),
            &Set(i) => ah.set(i),
            &While(ref ops) => {
                while ah.get() != 0 {
                    for op in ops.iter() {
                        op.execute(ah, fun_holder);
                    }
                }
            }
            &InsFuns => ah.set(fun_holder.no_functions() as i32 ),
            &CallFun => fun_holder.execute(ah),
            &CallFSep(args) => fun_holder.execute_separate(ah, args),
            &Read => ah.read(),
            &Write => ah.write(),
            &Debug => ah.debug(fun_holder.no_functions()),
            &MoveTo(ref places) => {
                let val = ah.get();
                if val == 0 {
                    return;
                }
                ah.set(0);
                for &(place, mult) in places.iter() {
                    ah.add_at(place, val*mult);
                }
            }
            &EmptyOp => {}
            &SkipMove(i) => ah.skip_move(i),
            &PeekStack => ah.add(STACK_HOLDER.lock().unwrap().peek()),
            &PopStack(i) => ah.add(STACK_HOLDER.lock().unwrap().pop(i)),
            &PushStack => STACK_HOLDER.lock().unwrap().push(ah.get()),
            &StackLen => ah.set(STACK_HOLDER.lock().unwrap().len() as i32),
        }
    }
}