extern crate Brainfuck;

use Brainfuck::Compiler;
use Brainfuck::run;
use std::collections::VecDeque;
use std::env;

fn main() {
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();
    let mut compiler = Compiler::new();
    while !args.is_empty() {
        compiler.compile_file(args.pop_front().unwrap());
        compiler.execute(true);
    }
    run(&mut compiler);
}
