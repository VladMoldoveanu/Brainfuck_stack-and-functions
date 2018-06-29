//! Holds a single structure: Reader, which facilitates reading scripts from
//! both files and Strings

use std::fs::File;
use std::io::prelude::*;

/// Simple structure for a consumable reader which can be easily passed around.
/// Can read from a file or just use a given String.
/// Keeps only the characters which are currently supported in the language syntax.
///
/// # Examples
/// ```
/// use Brainfuck::reader::Reader;
///
/// let s = String::from("/-dsff~|\\hfdghg//.gdfd,+<>54245245>:;;;;{}>");
/// let mut rdr = Reader::from_string(s);
/// let mut ss = String::with_capacity(35);
/// while rdr.has_next() {
///   ss.push(rdr.next());
/// }
/// assert_eq!(String::from("/-~|\\//.,+<>>>"), ss);
/// ```
///
#[derive(Debug)]
pub struct Reader {
    ch: Vec<char>, // Characters to iterate over
    pos: usize, // Current position of the iteration
}

impl Reader {
    /// Creates new Reader with characters from a given String
    ///
    /// Keeps only valid syntax
    ///
    /// # Examples
    /// ```
    /// use Brainfuck::reader::Reader;
    ///
    /// let s = String::from("This is an example, string.");
    /// //Creates a reader with the characters ',' and '.'
    /// let mut reader = Reader::from_string(s);
    /// ```
    ///
    pub fn from_string(s: String) -> Reader {
        Reader{
            ch: s.chars().filter(|ch| CHARS.contains(ch)).collect(),
            pos: 0,
        }
    }
    /// Creates new Reader with characters from a given file
    ///
    /// # Examples
    /// ```
    /// use Brainfuck::reader::Reader;
    ///
    /// let file = String::from("reader_test.txt");
    /// let mut reader = Reader::from_file(file);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Err(String)` if the file does not exist or cannot read from it.
    ///
    pub fn from_file(f: String) -> Result<Reader, String> {
        let mut file = File::open(f.clone());
        match file {
            Ok(ref mut ff) => {
                let mut contents = String::new();
                match ff.read_to_string(&mut contents) {
                    Ok(_) => Ok(Reader::from_string(contents)),
                    Err(_) => Err(format!("Could not read from file '{}'", f)),
                }

            }
            Err(_) => Err(format!("No such file: {}", f)),
        }

    }
    /// Returns the next character without consuming it.
    ///
    /// # Examples
    /// ```
    /// use Brainfuck::reader::Reader;
    ///
    /// //Creates a reader with the characters ',' and '.'
    /// let mut reader = Reader::from_string(String::from("Testing, peek."));
    /// assert_eq!(reader.peek(), ',');
    /// assert_eq!(reader.peek(), ',');
    /// ```
    /// # Panics
    ///
    /// Panics if the whole String/File is consumed (i.e. has_next() returns false).
    ///
    pub fn peek(&self) -> char {
        self.ch[self.pos]
    }
    /// Checks if there are still characters to be consumed.
    ///
    /// # Examples
    /// ```
    /// use Brainfuck::reader::Reader;
    ///
    /// //Creates a reader with the character '.'
    /// let mut reader = Reader::from_string(String::from("Testing has_next."));
    /// assert!(reader.has_next());
    /// reader.next();
    /// assert!(!reader.has_next());
    /// ```
    ///
    pub fn has_next(&self) -> bool {
        self.pos < self.ch.len()
    }
    /// Return the next character, consuming it.
    ///
    /// # Examples
    /// ```
    /// use Brainfuck::reader::Reader;
    ///
    /// //Creates a reader with the characters ',' and '.'
    /// let mut reader = Reader::from_string(String::from("Testing, next."));
    /// assert_eq!(reader.next(), ',');
    /// assert_eq!(reader.next(), '.');
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the whole String/File is consumed (i.e. has_next() returns false).
    ///
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
        let mut rdr = Reader::from_file(file).expect("Test file 'reader_test.txt' not found");
        let mut ss = String::with_capacity(35);
        while rdr.has_next() {
            ss.push(rdr.next());
        }
        assert_eq!(s, ss);
    }
}