//! The array on which the operations are executed

mod input_reader;
use self::input_reader::INPUT_READER;
use dispatcher::operation::stack_handler::STACK_HOLDER;

/// Resizable array with 'negative' entries
pub struct ArrayHandler {
    arr: Vec<i32>,
    pos: usize,
    displacement: usize,
}

impl ArrayHandler {
    /// Creates a new array with the specified size
    ///
    /// If `mid_array` is true, the start point is in the middle of the array,
    /// allowing going to the left at 0 cost
    pub fn new(capacity: usize, mid_array: bool) -> ArrayHandler {
        assert!(capacity > 0);
        if mid_array {
            ArrayHandler{
                arr: vec![0; capacity],
                pos: capacity/2,
                displacement: capacity/2,
            }
        } else {
            ArrayHandler{
                arr: vec![0; capacity],
                pos: 0,
                displacement: 0,
            }
        }
    }
    /// Adds to the current position
    pub fn add(&mut self, i: i32) {
        self.arr[self.pos] += i;
    }
    /// Adds at a position with displacement `pos`
    pub fn add_at(&mut self, pos: i32, val: i32) {
        self.holds(pos);
        let pos = if pos < 0 {
            self.pos - (-pos) as usize
        } else {
            self.pos + pos as usize
        };
        self.arr[pos] += val;
    }
    /// Moves i to the right (if i is negative it moves to the left)
    pub fn move_r(&mut self, i: i32) {
        self.holds(i);
        if i < 0 {
            self.pos -= (-i) as usize;
        } else {
            self.pos += i as usize;
        }
    }
    /// Set the element at the current position to `i`
    pub fn set(&mut self, i: i32) {
        self.arr[self.pos] = i;
    }
    /// Used for copying from the current array to `ah` `args` numbers
    pub fn set_more(&mut self, ah: &mut ArrayHandler, args: usize) {
        self.holds(args as i32);
        ah.holds(args as i32);
        for i in 0..(args + 1) {
            self.arr[self.pos + i] = ah.arr[ah.pos + i];
        }
    }
    // Makes sure there is enough size to the right for a move
    fn resize_right(&mut self) {
        let n = self.arr.capacity();
        let mut aux = vec![0; n*2];
        aux[..n].clone_from_slice(&self.arr);
        self.arr = aux;
    }
    // Makes sure there is enough size to the left for a move
    fn resize_left(&mut self) {
        let n = self.arr.capacity();
        let mut aux = vec![0; n*2];
        aux[n..].clone_from_slice(&self.arr);
        self.pos += n;
        self.displacement += n;
        self.arr = aux;
    }
    // Makes sure there is enough space to move by `offset`
    fn holds(&mut self, offset: i32) {
        while offset + (self.pos as i32) >= (self.arr.capacity() as i32) {
            self.resize_right();
        }
        let mut poss = self.pos as i32;
        while offset + poss < 0 {
            self.resize_left();
            poss = self.pos as i32;
        }
    }
    /// Get the value at the current position
    pub fn get(&self) -> i32 {
        self.arr[self.pos]
    }
    /// Read from the standard input to the current position
    pub fn read(&mut self) {
        self.set(INPUT_READER.lock().unwrap().next());
    }
    /// Write the char at the current position to the standard output
    pub fn write(&self) {
        print!("{}", (self.arr[self.pos] as u8) as char);
    }
    /// Print debug info
    pub fn debug(&self, funs: usize) {
        for i in 0..self.arr.len() {
            if self.arr[i] != 0 {
                let pos = (i as i64) - (self.displacement as i64);
                println!("{}: {}", pos, self.arr[i]);
            }
        }
        let pt = (self.pos as i64) - (self.displacement as i64);
        println!("Pointer at {}", pt);
        STACK_HOLDER.lock().unwrap().debug();
        println!("Number of functions: {}", funs);
    }
    /// Executes `SkipMove(offset)`
    pub fn skip_move(&mut self, offset: i32) {
        while self.arr[self.pos] != 0 {
            self.holds(offset);
            if offset < 0 {
                self.pos -= (-offset) as usize;
            } else {
                self.pos += offset as usize;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayHandler;
    #[test]
    fn add_positive() {
        let mut ah = ArrayHandler::new(2, true);
        ah.add(10);
        assert_eq!(ah.arr, [0, 10]);
    }
    #[test]
    fn add_negative() {
        let mut ah = ArrayHandler::new(2, true);
        ah.add(-10);
        assert_eq!(ah.arr, [0, -10]);
    }
    #[test]
    fn move_forward_no_resize() {
        let mut ah = ArrayHandler::new(10, true);
        ah.move_r(2);
        assert_eq!(ah.pos, 7);
        assert_eq!(ah.arr.capacity(), 10);
    }
    #[test]
    fn move_backward_no_resize() {
        let mut ah = ArrayHandler::new(10, true);
        ah.move_r(-2);
        assert_eq!(ah.pos, 3);
        assert_eq!(ah.arr.capacity(), 10);
    }
    #[test]
    fn move_forward_resize() {
        let mut ah = ArrayHandler::new(2, true);
        ah.add(10);
        ah.move_r(3);
        assert_eq!(ah.pos, 4);
        assert_eq!(ah.arr.capacity(), 8);
        assert_eq!(ah.arr, [0, 10, 0, 0, 0, 0, 0, 0])
    }
    #[test]
    fn move_backward_resize() {
        let mut ah = ArrayHandler::new(2, true);
        ah.add(10);
        ah.move_r(-4);
        assert_eq!(ah.pos, 3);
        assert_eq!(ah.arr.capacity(), 8);
        assert_eq!(ah.arr, [0, 0, 0, 0, 0, 0, 0, 10])
    }
    #[test]
    fn move_both_dir_resize() {
        let mut ah = ArrayHandler::new(2, true);
        ah.add(10);
        ah.move_r(1);
        ah.move_r(-3);
        assert_eq!(ah.pos, 3);
        assert_eq!(ah.arr.capacity(), 8);
        assert_eq!(ah.arr, [0, 0, 0, 0, 0, 10, 0, 0])
    }
}