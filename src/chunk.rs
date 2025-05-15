use std::fmt::Debug;

use crate::common::OpCode;

#[derive(Copy, Clone)]
pub union ChunkData {
    opcode: OpCode,
    data: u8,
}
impl Debug for ChunkData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", unsafe { OpCode::from(self.data) })?;
        write!(f, "{:?}", unsafe { self.data })
    }
}

#[derive(Debug)]
pub struct Chunk {
    data: Vec<ChunkData>,
    pointer: usize,
}

impl Chunk {
    pub fn new(data: Vec<ChunkData>) -> Chunk {
        Chunk { data, pointer: 0 }
    }
    pub fn add_opcode(&mut self, opcode: OpCode) {
        self.data.push(ChunkData { opcode });
    }
    pub fn add_byte(&mut self, data: u8) {
        self.data.push(ChunkData { data });
    }
    pub fn get_length(&mut self) -> usize {
        self.data.len()
    }
    pub fn add_chunk(&mut self, other: &mut Chunk) {
        self.data.append(other.data.as_mut());
    }
    #[inline]
    pub fn set_pointer(&mut self, pointer: usize) {
        self.pointer = pointer;
    }
    #[inline]
    pub fn get_pointer(&mut self) -> usize {
        self.pointer
    }
    #[inline]
    pub fn get_instruction(&mut self) -> (OpCode, &[u8]) {
        let oc: OpCode = unsafe { self.data[self.pointer].opcode };
        self.pointer += 1;
        match oc {
            OpCode::DropArr => {
                const DATA_SIZE: usize = 2;
                let byte_slice = unsafe {
                    std::slice::from_raw_parts(
                        self.data[self.pointer..self.pointer + DATA_SIZE].as_ptr() as *const u8,
                        DATA_SIZE,
                    )
                };
                self.pointer += DATA_SIZE;
                (oc, byte_slice)
            }
            OpCode::DropStr => {
                const DATA_SIZE: usize = 1;
                let byte_slice = unsafe {
                    std::slice::from_raw_parts(
                        self.data[self.pointer..self.pointer + DATA_SIZE].as_ptr() as *const u8,
                        DATA_SIZE,
                    )
                };
                self.pointer += DATA_SIZE;
                (oc, byte_slice)
            }
            OpCode::Constant => {
                const DATA_SIZE: usize = 1;
                let byte_slice = unsafe {
                    std::slice::from_raw_parts(
                        self.data[self.pointer..self.pointer + DATA_SIZE].as_ptr() as *const u8,
                        DATA_SIZE,
                    )
                };
                self.pointer += DATA_SIZE;
                (oc, byte_slice)
            }
            OpCode::FunctionCall => {
                const DATA_SIZE: usize = 2;
                let byte_slice = unsafe {
                    std::slice::from_raw_parts(
                        self.data[self.pointer..self.pointer + DATA_SIZE].as_ptr() as *const u8,
                        DATA_SIZE,
                    )
                };
                self.pointer += DATA_SIZE;
                (oc, byte_slice)
            }
            OpCode::StackLoadLocalVar => {
                const DATA_SIZE: usize = 1;
                let byte_slice = unsafe {
                    std::slice::from_raw_parts(
                        self.data[self.pointer..self.pointer + DATA_SIZE].as_ptr() as *const u8,
                        DATA_SIZE,
                    )
                };
                self.pointer += DATA_SIZE;
                (oc, byte_slice)
            }
            OpCode::Advance => {
                const DATA_SIZE: usize = 1;
                let byte_slice = unsafe {
                    std::slice::from_raw_parts(
                        self.data[self.pointer..self.pointer + DATA_SIZE].as_ptr() as *const u8,
                        DATA_SIZE,
                    )
                };
                self.pointer += DATA_SIZE;
                (oc, byte_slice)
            }
            OpCode::AdvanceIfFalse => {
                const DATA_SIZE: usize = 1;
                let byte_slice = unsafe {
                    std::slice::from_raw_parts(
                        self.data[self.pointer..self.pointer + DATA_SIZE].as_ptr() as *const u8,
                        DATA_SIZE,
                    )
                };
                self.pointer += DATA_SIZE;
                (oc, byte_slice)
            }
            OpCode::ConstructArray => {
                const DATA_SIZE: usize = 1;
                let byte_slice = unsafe {
                    std::slice::from_raw_parts(
                        self.data[self.pointer..self.pointer + DATA_SIZE].as_ptr() as *const u8,
                        DATA_SIZE,
                    )
                };
                self.pointer += DATA_SIZE;
                (oc, byte_slice)
            }
            _ => (oc, &[]),
        }
    }
}
