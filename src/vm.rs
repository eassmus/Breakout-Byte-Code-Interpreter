use crate::chunk::Chunk;
use crate::common::OpCode;
use crate::value::{PrintValWrapper, Type, Value, ValueUnion};
use std::boxed::ThinBox;
use std::mem::ManuallyDrop;
use std::ops::DerefMut;

pub struct VM {
    program_data: Vec<Chunk>,
    value_stack: Vec<Value>,
    constants: Vec<Value>,
    function_stack: Vec<(usize, Vec<Value>)>,
    main_pointer: Option<usize>,
    position_stack: Vec<usize>,
    main_type: Option<Type>,
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
            main_type: None,
        }
    }
    pub fn give_data(&mut self, data: Chunk) {
        self.program_data.push(data);
    }
    pub fn update_constants(&mut self, constants: &Vec<Value>) {
        self.constants.clear();
        for c in constants {
            self.constants.push(c.clone());
        }
    }
    pub fn set_main(&mut self, offset: usize, main_type: Type) {
        self.main_pointer = match self.main_pointer {
            None => Some(offset),
            Some(x) => Some(x + offset + 1),
        };
        self.main_type = Some(main_type);
    }
    pub fn run(&mut self) -> Result<(), String> {
        self.value_stack.clear();
        self.function_stack.clear();
        self.function_stack
            .push((self.main_pointer.unwrap(), Vec::new()));
        for item in self.program_data.iter_mut() {
            item.set_pointer(0);
        }
        loop {
            let (op, data) =
                self.program_data[self.function_stack.last().unwrap().0].get_instruction();
            match op {
                OpCode::Return => {
                    if self.function_stack.last().unwrap().0 == self.main_pointer.unwrap() {
                        self.function_stack.pop();
                        println!(
                            "{}",
                            PrintValWrapper {
                                val: self.value_stack.last().unwrap(),
                                t: self.main_type.as_ref().unwrap()
                            }
                        );
                        return Ok(());
                    }
                    self.function_stack.pop();
                    self.program_data[self.function_stack.last().unwrap().0]
                        .set_pointer(self.position_stack.pop().unwrap());
                }
                OpCode::Constant => {
                    let constant = &self.constants[data[0] as usize];
                    self.value_stack_push(&[constant.clone()]);
                }
                OpCode::AddI => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.i += b.value.i;
                    }
                }
                OpCode::AddF => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.f += b.value.f;
                    }
                }
                OpCode::SubtractI => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.i -= b.value.i;
                    }
                }
                OpCode::SubtractF => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.f -= b.value.f;
                    }
                }
                OpCode::MultiplyI => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.i *= b.value.i;
                    }
                }
                OpCode::MultiplyF => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.f *= b.value.f;
                    }
                }
                OpCode::DivideI => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.i /= b.value.i;
                    }
                }
                OpCode::DivideF => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.f /= b.value.f;
                    }
                }
                OpCode::True => self.value_stack_push(&[Value {
                    value: ValueUnion { b: true },
                }]),
                OpCode::False => self.value_stack_push(&[Value {
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
                        .push(self.function_stack.last().unwrap().1[data[0] as usize].clone());
                }
                OpCode::Not => {
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = !a.value.b;
                    }
                }
                OpCode::EqualI => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.i == b.value.i;
                    }
                }
                OpCode::EqualF => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.f == b.value.f;
                    }
                }
                OpCode::EqualB => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.b == b.value.b;
                    }
                }
                OpCode::EqualS => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.s.as_str() == b.value.s.as_str();
                    }
                }
                OpCode::LessThanI => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.i < b.value.i;
                    }
                }
                OpCode::LessThanF => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.f < b.value.f;
                    }
                }
                OpCode::GreaterThanI => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.i > b.value.i;
                    }
                }
                OpCode::GreaterThanF => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.f > b.value.f;
                    }
                }
                OpCode::LessThanOrEqualI => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.i <= b.value.i;
                    }
                }
                OpCode::LessThanOrEqualF => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.f <= b.value.f;
                    }
                }
                OpCode::GreaterThanOrEqualI => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.i >= b.value.i;
                    }
                }
                OpCode::GreaterThanOrEqualF => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        a.value.b = a.value.f >= b.value.f;
                    }
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
                OpCode::ConstructArray => {
                    let size = data[0] as usize;
                    let mut arr = Vec::new();
                    arr.resize(size, ValueUnion { i: 0 });
                    for i in 0..size {
                        arr[size - i - 1] = self.value_stack_pop().value;
                    }
                    arr.reverse();
                    self.value_stack_push(&[Value {
                        value: ValueUnion {
                            a: ManuallyDrop::new(ThinBox::new(arr)),
                        },
                    }]);
                }
                OpCode::ConcatArr => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        let a_arr: &mut Vec<ValueUnion> = a.value.a.deref_mut();
                        let b_arr = b.value.a.as_ref();
                        a_arr.extend_from_slice(b_arr);
                    }
                }
                OpCode::ConcatStr => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        let a_str: &mut String = a.value.s.deref_mut();
                        let b_str = b.value.s.as_str();
                        a_str.push_str(b_str);
                    }
                }
                OpCode::Index => {
                    let b = self.value_stack_pop();
                    let a = self.value_stack_last_mut();
                    unsafe {
                        let a_arr: &mut Vec<ValueUnion> = a.value.a.deref_mut();
                        let b_val = b.value.i;
                        a.value = a_arr[b_val as usize].clone();
                    }
                }
                OpCode::NullCode => {
                    panic!("NullCode");
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
