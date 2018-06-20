#[derive(Debug)]
pub struct CmdChars {
    chs: Vec<char>,
    pos: usize,
}

impl CmdChars {
    pub fn new(s: String) -> CmdChars {
        CmdChars {
            chs: s.chars().collect(),
            pos: 0,
        }
    }
    pub fn peek(&self) -> Option<char> {
        if self.pos >= self.chs.len() {
            return None
        }
        Some(self.chs[self.pos])
    }
    pub fn next(&mut self) {
        self.pos += 1
    }
    pub fn push(&mut self, s: String) {
        self.chs = s.chars().collect();
        self.pos = 0;
    }
}