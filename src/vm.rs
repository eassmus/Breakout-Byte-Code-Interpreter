use crate::chunk::Chunk;
use crate::common::OpCode;
use crate::value::{PrimType, Value, ValueUnion};

pub struct VM {
    program_data: Vec<Chunk>,
    value_stack: Vec<Value>,
    constants: Vec<Value>,
    function_stack: Vec<usize>,
    main_pointer: Option<usize>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            program_data: Vec::new(),
            value_stack: Vec::new(),
            constants: Vec::new(),
            function_stack: Vec::new(),
            main_pointer: None,
        }
    }
    pub fn give_data(&mut self, data: Chunk) {
        println!("{:?}", data);
        self.program_data.push(data);
    }
    pub fn give_constants(&mut self, constants: Vec<Value>) {
        for c in constants {
            self.constants.push(c);
        }
    }
    pub fn set_main(&mut self, main_pointer: usize) {
        self.main_pointer = Some(main_pointer);
    }
    pub fn run(&mut self) -> Result<(), String> {
        self.function_stack.push(self.main_pointer.unwrap());
        loop {
            let (op, data) =
                self.program_data[*self.function_stack.last().unwrap()].get_instruction();
            match op {
                OpCode::Return => {
                    if self.function_stack.last().unwrap() == &self.main_pointer.unwrap() {
                        self.function_stack.pop();
                        println!("{:?}", self.value_stack.last().unwrap().value);
                        return Ok(());
                    }
                    self.function_stack.pop();
                }
                OpCode::Constant => {
                    let constant = self.constants[data[0] as usize];
                    self.value_stack_push(&[constant]);
                }
                OpCode::Add => {
                    let mut a = self.value_stack_pop();
                    let b = self.value_stack_pop();
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
                    self.value_stack_push(&[a]);
                }
                OpCode::Subtract => {
                    let mut a = self.value_stack_pop();
                    let b = self.value_stack_pop();
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
                    self.value_stack_push(&[a]);
                }
                OpCode::Multiply => {
                    let mut a = self.value_stack_pop();
                    let b = self.value_stack_pop();
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
                    self.value_stack_push(&[a]);
                }
                OpCode::Divide => {
                    let mut a = self.value_stack_pop();
                    let b = self.value_stack_pop();
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
                    self.value_stack_push(&[a]);
                }
                OpCode::True => self.value_stack_push(&[Value {
                    t: PrimType::Bool,
                    value: ValueUnion { b: true },
                }]),
                OpCode::False => self.value_stack_push(&[Value {
                    t: PrimType::Bool,
                    value: ValueUnion { b: false },
                }]),

                _ => todo!(),
            }
        }
    }
    #[inline]
    fn value_stack_push(&mut self, slice: &[Value]) {
        self.value_stack.extend_from_slice(slice);
    }
    #[inline]
    fn value_stack_pop(&mut self) -> Value {
        self.value_stack.pop().unwrap()
    }
    #[inline]
    fn value_stack_count(&self) -> usize {
        self.value_stack.len()
    }
}
