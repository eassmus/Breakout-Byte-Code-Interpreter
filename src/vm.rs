use crate::chunk::{Chunk, ChunkData};
use crate::common::OpCode;
use crate::value::{PrimType, Value, ValueUnion};
use std::error::Error;
use std::fmt;

pub struct VM {
    program_data: Chunk,
    stack: Vec<Value>,
    constants: Vec<Value>,
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
                    let mut top = self.stack_pop();
                    unsafe {
                        match top.t {
                            PrimType::Float => {
                                top.value.f = -top.value.f;
                            }
                            PrimType::Int => {
                                top.value.i = -top.value.i;
                            }
                            PrimType::Bool => {
                                top.value.b = !top.value.b;
                            }
                            _ => panic!(),
                        }
                    }
                    self.stack_push(&[top]);
                }
                OpCode::Add => {
                    let mut a = self.stack_pop();
                    let b = self.stack_pop();
                    unsafe {
                        match a.t {
                            PrimType::Float => {
                                a.value.f += b.value.f;
                            }
                            PrimType::Int => {
                                a.value.i += b.value.i;
                            }
                            _ => panic!(),
                        }
                    }
                    self.stack_push(&[a]);
                }
                OpCode::Subtract => {
                    let mut a = self.stack_pop();
                    let b = self.stack_pop();
                    unsafe {
                        match a.t {
                            PrimType::Float => {
                                a.value.f -= b.value.f;
                            }
                            PrimType::Int => {
                                a.value.i -= b.value.i;
                            }
                            _ => panic!(),
                        }
                    }
                    self.stack_push(&[a]);
                }
                OpCode::Multiply => {
                    let mut a = self.stack_pop();
                    let b = self.stack_pop();
                    unsafe {
                        match a.t {
                            PrimType::Float => {
                                a.value.f *= b.value.f;
                            }
                            PrimType::Int => {
                                a.value.i *= b.value.i;
                            }
                            _ => panic!(),
                        }
                    }
                    self.stack_push(&[a]);
                }
                OpCode::Divide => {
                    let mut a = self.stack_pop();
                    let b = self.stack_pop();
                    unsafe {
                        match a.t {
                            PrimType::Float => {
                                a.value.f /= b.value.f;
                            }
                            PrimType::Int => {
                                a.value.i /= b.value.i;
                            }
                            _ => panic!(),
                        }
                    }
                    self.stack_push(&[a]);
                }
                OpCode::True => self.stack_push(&[Value {
                    t: PrimType::Bool,
                    value: ValueUnion { b: true },
                }]),
                OpCode::False => self.stack_push(&[Value {
                    t: PrimType::Bool,
                    value: ValueUnion { b: false },
                }]),
                _ => todo!(),
            }
        }
        Ok(())
    }
    fn stack_push(&mut self, slice: &[Value]) {
        self.stack.extend_from_slice(slice);
    }
    fn stack_pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }
    #[inline]
    fn stack_count(&self) -> usize {
        self.stack.len()
    }
}
