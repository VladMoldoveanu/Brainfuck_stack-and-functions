mod input_reader;
use self::input_reader::INPUT_READER;
use dispatcher::operation::stack_handler::STACK_HOLDER;

pub struct ArrayHandler {
    arr: Vec<i32>,
    pos: usize,
    displacement: usize,
}

impl ArrayHandler {
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
    pub fn add(&mut self, i: i32) {
        self.arr[self.pos] += i;
    }
    pub fn add_at(&mut self, pos: i32, val: i32) {
        self.holds(pos);
        let pos = if pos < 0 {
            self.pos - (-pos) as usize
        } else {
            self.pos + pos as usize
        };
        self.arr[pos] += val;
    }
    pub fn move_r(&mut self, i: i32) {
        self.holds(i);
        if i < 0 {
            self.pos -= (-i) as usize;
        } else {
            self.pos += i as usize;
        }
    }
    pub fn set(&mut self, i: i32) {
        self.arr[self.pos] = i;
    }
    pub fn set_more(&mut self, ah: &mut ArrayHandler, args: usize) {
        self.holds(args as i32);
        ah.holds(args as i32);
        for i in 0..(args + 1) {
            self.arr[self.pos + i] = ah.arr[ah.pos + i];
        }
    }
    fn resize_right(&mut self) {
        let n = self.arr.capacity();
        let mut aux = vec![0; n*2];
        aux[..n].clone_from_slice(&self.arr);
        self.arr = aux;
    }
    fn resize_left(&mut self) {
        let n = self.arr.capacity();
        let mut aux = vec![0; n*2];
        aux[n..].clone_from_slice(&self.arr);
        self.pos += n;
        self.displacement += n;
        self.arr = aux;
    }
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
    pub fn get(&self) -> i32 {
        self.arr[self.pos]
    }
    pub fn read(&mut self) {
        self.set(INPUT_READER.lock().unwrap().next());
    }
    pub fn write(&self) {
        print!("{}", (self.arr[self.pos] as u8) as char);
    }
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