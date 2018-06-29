//! Dispatcher for the operations. Main functionality is parsing a script
//! and returning the unoptimised operations.

pub mod operation;
use reader::Reader;
pub use self::operation::*;
pub use self::Dispatcher::*;
use optimiser::loop_optimiser;

///Last type of operation found
pub enum Dispatcher{
    ///Found a simple Operation
    Op(Operation),
    ///Defined a function
    Fun,
    ///Finished the script
    Empty,
    ///Found an error in the script
    Error(String),
}

///Parses the next character (and more if necessary) and returns the corresponding operation
pub fn dispatch(rd: &mut Reader, fh: &mut FunctionHolder) -> Dispatcher {
    if !rd.has_next() {
        return Empty;
    }
    match rd.peek() {
        '>' => return Op(Move(amalgamate(rd))),
        '<' => return Op(Move(-amalgamate(rd))),
        '+' => return Op(Add(amalgamate(rd))),
        '-' => return Op(Add(-amalgamate(rd))),
        '.' => {rd.next(); return Op(Write);}
        ',' => {rd.next(); return Op(Read);}
        '!' => {
            let n = amalgamate(rd);
            if n > 1 {
                return Op(Set(fh.curr_funs()));
            } else {
                return Op(InsFuns);
            }
        }
        '/' => {
            rd.next();
            if !rd.has_next() {
                return Op(CallFSep(0));
            }
            match rd.peek() {
                '\\' => return Op(CallFSep(amalgamate(rd) as usize)),
                _ => return Op(CallFSep(0)),
            }
        }
        '|' => {rd.next(); return Op(CallFun);}
        ']' => {rd.next(); return Error(String::from("Found ']' without matching '['"))}
        '\\' => {rd.next(); return Error(String::from("Found '\\' without '/' function call"))}
        '[' => {rd.next(); return create_loop(rd, fh)}
        '~' => {rd.next(); return create_fun(rd, fh)}
        '#' => {amalgamate(rd); return Op(Debug)}
        '@' => {amalgamate(rd); return Op(PeekStack)}
        '&' => {rd.next(); return Op(PushStack)}
        '^' => {rd.next(); return Op(PopStack(1))}
        '?' => {amalgamate(rd); return Op(StackLen)}
        _ => return Error(String::from("Unidentified character passed filtering")),
    }
}
/// Collapses multiple operations of the same kind
fn amalgamate(rd: &mut Reader) -> i32 {
    let mut no= 0;
    let ch = rd.peek();
    while rd.has_next() && ch == rd.peek() {
        no += 1;
        rd.next();
    }
    no
}
/// Creates a loop, reads until it finds a corresponding ']'
///
/// # Errors
/// Returns an error if the loop has no matching ']' or a function is declared inside.
fn create_loop(rd: &mut Reader, fh: &mut FunctionHolder) -> Dispatcher {
    let mut ops: Vec<Operation> = vec![];
    while rd.has_next() && rd.peek() != ']' {
        match rd.peek() {
            '~' => return Error(String::from("Error: functions cannot be declared in loops")),
            _ => {
                match dispatch(rd, fh) {
                    Op(op) => ops.push(op),
                    Fun => return Error(String::from("Error: functions cannot be declared in loops")),
                    Error(s) => return Error(s),
                    Empty => return Error(String::from("Error: Dispatch returned empty from non-empty reader")),
                }
            }
        }
    }
    if !rd.has_next() {
        return Error(String::from("Error: loop has no end point"));
    }
    rd.next();
    Op(loop_optimiser(ops))
}
///Creates a function. Reads until it finds a matching '~'
///
/// # Errors
/// Returns an error if the function has no end point
fn create_fun(rd: &mut Reader, fh: &mut FunctionHolder) -> Dispatcher {
    let mut ops: Vec<Operation> = vec![];
    while rd.has_next() && rd.peek() != '~' {
        match dispatch(rd, fh) {
            Op(op) => ops.push(op),
            Fun => return Error(String::from("Error: Function inside a function: HOW????")),
            Error(s) => return Error(s),
            Empty => return Error(String::from("Error: Dispatch returned empty from non-empty reader")),
        }
    }
    if !rd.has_next() {
        return Error(String::from("Error: function has no end point"));
    }
    rd.next();
    fh.add_temp(ops);
    Fun
}