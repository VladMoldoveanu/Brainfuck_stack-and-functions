use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Reader {
    ch: Vec<char>,
    pos: usize,
}

impl Reader {
    pub fn from_string(s: String) -> Reader {
        Reader{
            ch: s.chars().filter(|ch| CHARS.contains(ch)).collect(),
            pos: 0,
        }
    }
    pub fn from_file(f: String) -> Reader {
        let mut file = File::open(f.clone())
            .expect(&format!("File not found: {}", f));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Problem reading from file.");
        Reader::from_string(contents)
    }
    pub fn peek(&self) -> char {
        self.ch[self.pos]
    }
    pub fn has_next(&self) -> bool {
        self.pos < self.ch.len()
    }
    pub fn next(&mut self) -> char {
        self.pos += 1;
        self.ch[self.pos-1]
    }
}

lazy_static! {
    static ref CHARS: Vec<char> = vec!['>', '<', '-', '+', '.', ',', '~', '!', '/', '\\', '|', '[', ']', '#', '@', '&', '^', '?'];
}

#[cfg(test)]
mod tests {
    use super::Reader;
    #[test]
    fn go_through() {
        let s = String::from("/-~||||\\\\////...,,,++++<><><>>>><>");
        let mut rdr = Reader::from_string(s.clone());
        let mut ss = String::with_capacity(35);
        while rdr.has_next() {
            ss.push(rdr.next());
        }
        assert_eq!(s, ss);
    }
    #[test]
    fn filtering() {
        let s = String::from("/-dsff~||||\\\\hfdghg////...gdfd,,,++++<><><>54245245>>><:::;;;;{}>");
        let mut rdr = Reader::from_string(s);
        let mut ss = String::with_capacity(35);
        while rdr.has_next() {
            ss.push(rdr.next());
        }
        assert_eq!(String::from("/-~||||\\\\////...,,,++++<><><>>>><>"), ss);
    }
    #[test]
    fn read_file() {
        let s = String::from("[],...!<><><<<>++++++//||||\\\\\\~~~~~~~");
        let file = String::from("reader_test.txt");
        let mut rdr = Reader::from_file(file);
        let mut ss = String::with_capacity(35);
        while rdr.has_next() {
            ss.push(rdr.next());
        }
        assert_eq!(s, ss);
    }
}