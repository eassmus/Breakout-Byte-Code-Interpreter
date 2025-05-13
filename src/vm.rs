use crate::chunk::{Chunk, ChunkData};
use crate::common::OpCode;
use std::error::Error;
use std::fmt;

pub struct VM {
    program_data: Chunk,
    stack: Vec<i64>,
    constants: Vec<i64>,
}

enum VMErrorData {
    RuntimeError(),
    CompileError(),
}
pub struct VMError {
    message: String,
    data: VMErrorData,
}
impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl fmt::Debug for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for VMError {}

impl VM {
    pub fn new() -> VM {
        VM {
            program_data: Chunk::new(Vec::new()),
            stack: Vec::new(),
            constants: Vec::new(),
        }
    }
    pub fn give_data(&mut self, _data: &mut Vec<ChunkData>) {
        self.program_data.add_data(_data);
    }
    pub fn run(&mut self) -> Result<(), VMError> {
        loop {
            let (op, data) = self.program_data.get_instruction();
            match op {
                OpCode::Return => break,
                OpCode::Constant => {
                    let constant = self.constants[data[0] as usize];
                    self.stack_push(&[constant]);
                }
                OpCode::Negate => {
                    let top = self.stack_pop();
                    self.stack_push(&[-top]);
                }
                OpCode::Add => {
                    let b = self.stack_pop();
                    let a = self.stack_pop();
                    self.stack_push(&[a + b]);
                }
                OpCode::Subtract => {
                    let b = self.stack_pop();
                    let a = self.stack_pop();
                    self.stack_push(&[a - b]);
                }
                OpCode::Multiply => {
                    let b = self.stack_pop();
                    let a = self.stack_pop();
                    self.stack_push(&[a * b]);
                }
                OpCode::Divide => {
                    let b = self.stack_pop();
                    let a = self.stack_pop();
                    self.stack_push(&[a / b]);
                }
                _ => todo!(),
            }
        }
        Ok(())
    }
    fn stack_push(&mut self, slice: &[i64]) {
        self.stack.extend_from_slice(slice);
    }
    fn stack_pop(&mut self) -> i64 {
        self.stack.pop().unwrap()
    }
    #[inline]
    fn stack_count(&self) -> usize {
        self.stack.len()
    }
}
