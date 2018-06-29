//! Simple String consumer for easy input handling

#[derive(Debug)]
pub struct CmdChars {
    chs: Vec<char>,
    pos: usize,
}

impl CmdChars {
    /// Creates a new Char sequence from a given string
    pub fn new(s: String) -> CmdChars {
        CmdChars {
            chs: s.chars().collect(),
            pos: 0,
        }
    }
    /// Returns the next char without consuming it,
    /// or none if there is no char left
    pub fn peek(&self) -> Option<char> {
        if self.pos >= self.chs.len() {
            return None
        }
        Some(self.chs[self.pos])
    }
    /// Advances to the next char without returning it
    pub fn next(&mut self) {
        self.pos += 1
    }
    /// Replaces the current sequence with a new one
    pub fn push(&mut self, s: String) {
        self.chs = s.chars().collect();
        self.pos = 0;
    }
}