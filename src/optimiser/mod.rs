use dispatcher::operation::base_operation::{Operation::*, Operation};
use std::collections::{VecDeque, HashMap};

pub fn optimise_code(ops: VecDeque<Operation>) -> VecDeque<Operation> {
    let mut optimised: Vec<Operation> = Vec::with_capacity(ops.len());
    for i in ops.into_iter() {
        reduce_top(&mut optimised, i);
    }
    optimised.into_iter().collect()
}

fn reduce_top(ops: &mut Vec<Operation>, op: Operation) {
    if let EmptyOp = op {
        return;
    }
    if ops.len() == 0 {
        ops.push(op);
        return;
    }
    match op {
        Add(i) => {
            if let Add(j) = ops[ops.len() - 1] {
                ops.pop();
                if i + j != 0 {
                    ops.push(Add(i+j));
                }
            } else if let Set(j) = ops[ops.len() - 1] {
                ops.pop();
                ops.push(Set(i + j));
            } else { ops.push(Add(i)); }
        }
        Move(i) => {
            if let Move(j) = ops[ops.len() - 1] {
                ops.pop();
                if i + j != 0 {
                    ops.push(Move(i+j));
                }
            } else { ops.push(Move(i)); }
        }
        Set(i) => {
            if let Add(_) = ops[ops.len() - 1] {
                ops.pop();
            } else if let Set(_) = ops[ops.len() - 1] {
                ops.pop();
            } else if let PeekStack = ops[ops.len() - 1] {
                ops.pop();
            } else if let StackLen = ops[ops.len() - 1] {
                ops.pop();
            }
            ops.push(Set(i));
        }
        Read => {
            if let Add(_) = ops[ops.len() - 1] {
                ops.pop();
            } else if let Set(_) = ops[ops.len() - 1] {
                ops.pop();
            } else if let PeekStack = ops[ops.len() - 1] {
                ops.pop();
            } else if let StackLen = ops[ops.len() - 1] {
                ops.pop();
            }
            ops.push(Read);
        }
        While(w_ops) => {
            if let While(_) = ops[ops.len() - 1] {

            } else if let Set(0) = ops[ops.len() - 1] {

            } else if let MoveTo(_) = ops[ops.len() - 1] {

            } else if let SkipMove(_) = ops[ops.len() - 1] {

            } else {
                ops.push(While(w_ops));
            }
        }
        PopStack(i) => {
            if let PopStack(j) = ops[ops.len() - 1] {
                ops.pop();
                ops.push(PopStack(i + j));
            } else {
                ops.push(PopStack(i));
            }
        }
        StackLen => {
            if let Add(_) = ops[ops.len() - 1] {
                ops.pop();
            } else if let Set(_) = ops[ops.len() - 1] {
                ops.pop();
            } else if let PeekStack = ops[ops.len() - 1] {
                ops.pop();
            } else if let StackLen = ops[ops.len() - 1] {
                ops.pop();
            }
            ops.push(StackLen);
        }
        x => ops.push(x),
    }
}

pub fn loop_optimiser(ops: Vec<Operation>) -> Operation {
    let mut optimised: Vec<Operation> = Vec::with_capacity(ops.len());
    for i in ops.into_iter() {
        reduce_top(&mut optimised, i);
    }
    if optimised.len() == 0 {
        return EmptyOp;
    }
    if optimised.len() == 1 {
        if let Add(i) = optimised[0] {
            if i == 1 || i == -1 {
                return Set(0);
            }
        } else if let Set(i) = optimised[0] {
            if i == 0 {
                return Set(0);
            }
        } else if let Move(i) = optimised[0] {
            return SkipMove(i);
        }
    }
    if move_and_add(&optimised) {
        let mut points: HashMap<i32, i32> = HashMap::new();
        let mut curr_pos = 0;
        for op in optimised {
            match op {
                Add(i) => {
                    if curr_pos != 0 {
                        let counter = points.entry(curr_pos).or_insert(0);
                        *counter += i;
                    }
                }
                Move(i) => curr_pos += i,
                x => panic!("Found {:?} in add_move!", x),
            }
        }
        return MoveTo(points.into_iter().collect());
    }
    While(optimised)
}

fn move_and_add(ops: &Vec<Operation>) -> bool {
    let mut total_move = 0;
    let mut point_diff = 0;
    for i in 0..ops.len() {
        if let Add(j) = ops[i] {
            if total_move == 0 {
                point_diff += j;
            }
        } else if let Move(j) = ops[i]{
            total_move += j;
        } else {
            return false;
        }
    }
    return total_move == 0 && point_diff == -1;
}