use console::Term;

use crate::parser::Instruction;

const ARR_SIZE: usize = 30000;

pub struct BFEnv {
    array: [i8; ARR_SIZE],
    ptr: usize,
    term: Term,
}

impl BFEnv {
    pub fn new() -> BFEnv {
        BFEnv {
            array: [0; ARR_SIZE],
            ptr: 0,
            term: Term::stdout(),
        }
    }

    pub fn run(&mut self, instructions: &[impl Instruction]) {
        for i in instructions {
            i.execute(self);
        }
    }

    pub fn incr_ptr(&mut self) {
        self.ptr += 1;
        if self.ptr >= ARR_SIZE {
            panic!("RANGE ERROR: Pointer incremented past array bounds");
        }
    }

    pub fn decr_ptr(&mut self) {
        if self.ptr <= 0 {
            panic!("RANGE ERROR: Pointer decremented under 0");
        }
        self.ptr -= 1;
    }

    pub fn incr_arr(&mut self) {
        self.array[self.ptr] += 1;
    }

    pub fn decr_arr(&mut self) {
        self.array[self.ptr] -= 1;
    }

    pub fn get_byte(&self) -> u8 {
        self.array[self.ptr] as u8
    }

    pub fn read_byte(&mut self) {
        let input = self.term.read_char().expect("Error reading char");
        self.array[self.ptr] = input as u8 as i8;
    }
}
