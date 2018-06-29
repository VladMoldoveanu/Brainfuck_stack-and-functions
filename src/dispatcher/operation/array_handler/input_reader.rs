//! Holds the global input reader for the script operations

use std::io::stdin;
use std::sync::Mutex;

/// Structure which holds the last line and consumes characters one by one
pub struct InputReader {
    ch: Vec<char>,
    pos: usize,
}

impl InputReader{
    /// Constructs a new InputReader
    pub fn new() -> InputReader {
        InputReader {
            ch: vec!(),
            pos: 0,
        }
    }
    // Read when there are no characters left
    fn read(&mut self) {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        self.ch = input.chars().collect();
        self.pos = 0;
    }
    /// Returns the next character from the input as i32
    pub fn next(&mut self) -> i32 {
        if self.pos == self.ch.len() {
            self.read();
        }
        self.pos += 1;
        self.ch[self.pos - 1] as i32
    }
}

lazy_static! {
    /// The global input reader for script operations
    pub static ref INPUT_READER: Mutex<InputReader> = Mutex::new(InputReader::new());
}