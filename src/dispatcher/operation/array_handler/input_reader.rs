use std::io::stdin;
use std::sync::Mutex;

pub struct InputReader {
    ch: Vec<char>,
    pos: usize,
}

impl InputReader{
    pub fn new() -> InputReader {
        InputReader {
            ch: vec!(),
            pos: 0,
        }
    }
    fn read(&mut self) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        self.ch = input.chars().collect();
        self.pos = 0;
    }
    pub fn next(&mut self) -> i32 {
        if self.pos == self.ch.len() {
            self.read();
        }
        self.pos += 1;
        self.ch[self.pos - 1] as i32
    }
}

lazy_static! {
    pub static ref INPUT_READER: Mutex<InputReader> = Mutex::new(InputReader::new());
}