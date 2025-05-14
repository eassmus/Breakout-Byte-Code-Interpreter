use crate::chunk::Chunk;
use crate::common::OpCode;
use crate::value::{PrimType, Value, ValueUnion};

pub struct VM {
    program_data: Vec<Chunk>,
    value_stack: Vec<Value>,
    constants: Vec<Value>,
    function_stack: Vec<(usize, Vec<Value>)>,
    main_pointer: Option<usize>,
    position_stack: Vec<usize>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            program_data: Vec::new(),
            value_stack: Vec::new(),
            constants: Vec::new(),
            function_stack: Vec::new(),
            main_pointer: None,
            position_stack: Vec::new(),
        }
    }
    pub fn give_data(&mut self, data: Chunk) {
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
        self.function_stack
            .push((self.main_pointer.unwrap(), Vec::new()));
        loop {
            let (op, data) =
                self.program_data[self.function_stack.last().unwrap().0].get_instruction();
            match op {
                OpCode::Return => {
                    if self.function_stack.last().unwrap().0 == self.main_pointer.unwrap() {
                        self.function_stack.pop();
                        println!("{:?}", self.value_stack.last().unwrap());
                        return Ok(());
                    }
                    self.function_stack.pop();
                    self.program_data[self.function_stack.last().unwrap().0]
                        .set_pointer(self.position_stack.pop().unwrap());
                }
                OpCode::Constant => {
                    let constant = self.constants[data[0] as usize];
                    self.value_stack_push(&[constant]);
                }
                OpCode::Add => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
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
                }
                OpCode::Subtract => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
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
                }
                OpCode::Multiply => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
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
                }
                OpCode::Divide => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
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
                }
                OpCode::True => self.value_stack_push(&[Value {
                    t: PrimType::Bool,
                    value: ValueUnion { b: true },
                }]),
                OpCode::False => self.value_stack_push(&[Value {
                    t: PrimType::Bool,
                    value: ValueUnion { b: false },
                }]),
                OpCode::FunctionCall => {
                    let next_func = data[0] as usize;
                    let mut args = Vec::new();
                    for _ in 0..data[1] {
                        args.push(self.value_stack_pop());
                    }
                    args.reverse();
                    self.position_stack.push(
                        self.program_data[self.function_stack.last().unwrap().0].get_pointer(),
                    );
                    self.function_stack.push((next_func, args));
                    self.program_data[self.function_stack.last().unwrap().0].set_pointer(0);
                }
                OpCode::StackLoadLocalVar => {
                    self.value_stack
                        .push(self.function_stack.last().unwrap().1[data[0] as usize]);
                }
                OpCode::Not => {
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = !a.value.b;
                    }
                }
                OpCode::Equal => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.i == b.value.i;
                        a.t = PrimType::Bool;
                    }
                }
                OpCode::NotEqual => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.i != b.value.i;
                        a.t = PrimType::Bool;
                    }
                }
                OpCode::LessThan => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        match a.t {
                            PrimType::Float => {
                                a.value.b = a.value.f < b.value.f;
                            }
                            PrimType::Int => {
                                a.value.b = a.value.i < b.value.i;
                            }
                            _ => panic!(),
                        }
                    }
                    a.t = PrimType::Bool;
                }
                OpCode::GreaterThan => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        match a.t {
                            PrimType::Float => {
                                a.value.b = a.value.f > b.value.f;
                            }
                            PrimType::Int => {
                                a.value.b = a.value.i > b.value.i;
                            }
                            _ => panic!(),
                        }
                    }
                    a.t = PrimType::Bool;
                }
                OpCode::LessThanOrEqual => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        match a.t {
                            PrimType::Float => {
                                a.value.b = a.value.f <= b.value.f;
                            }
                            PrimType::Int => {
                                a.value.b = a.value.i <= b.value.i;
                            }
                            _ => panic!(),
                        }
                    }
                    a.t = PrimType::Bool;
                }
                OpCode::GreaterThanOrEqual => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        match a.t {
                            PrimType::Float => {
                                a.value.b = a.value.f >= b.value.f;
                            }
                            PrimType::Int => {
                                a.value.b = a.value.i >= b.value.i;
                            }
                            _ => panic!(),
                        }
                    }
                    a.t = PrimType::Bool;
                }
                OpCode::Advance => {
                    let amount = data[0] as usize;
                    let current =
                        self.program_data[self.function_stack.last().unwrap().0].get_pointer();
                    self.program_data[self.function_stack.last().unwrap().0]
                        .set_pointer(current + amount);
                }
                OpCode::AdvanceIfFalse => {
                    let amount = data[0] as usize;
                    let a = self.value_stack_pop();
                    if !unsafe { a.value.b } {
                        let current =
                            self.program_data[self.function_stack.last().unwrap().0].get_pointer();
                        self.program_data[self.function_stack.last().unwrap().0]
                            .set_pointer(current + amount);
                    }
                }
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
    fn value_stack_last_mut(&mut self) -> &mut Value {
        self.value_stack.last_mut().unwrap()
    }
}
