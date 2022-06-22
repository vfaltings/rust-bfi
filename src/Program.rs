use crate::parser::Instruction;

const ARR_SIZE: usize = 30000;

pub struct BFEnv {
    array: [i8; 30000],
    ptr: usize,
}

impl BFEnv {
    pub fn new() -> BFEnv {
        BFEnv {
            array: [0; 30000],
            ptr: 0,
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
        todo!()
    }
}
