use crate::parser::Literal;
use ordered_float::OrderedFloat;
use std::boxed::ThinBox;
use std::mem::ManuallyDrop;

#[derive(Debug, Clone, Copy)]
pub enum PrimType {
    Float,
    Int,
    Bool,
    String,
    Array,
}

pub union ValueUnion {
    pub f: OrderedFloat<f64>,
    pub i: i64,
    pub b: bool,
    pub a: ManuallyDrop<ThinBox<(PrimType, Vec<ValueUnion>)>>,
    pub s: ManuallyDrop<ThinBox<String>>,
}

impl Clone for ValueUnion {
    fn clone(&self) -> Self {
        unsafe { std::mem::transmute_copy(self) }
    }
}

#[derive(Clone)]
pub struct Value {
    pub t: PrimType,
    pub value: ValueUnion,
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.t {
            PrimType::Float => write!(f, "{}", unsafe { self.value.f }),
            PrimType::Int => write!(f, "{}", unsafe { self.value.i }),
            PrimType::Bool => write!(f, "{}", unsafe { self.value.b }),
            PrimType::String => write!(f, "{}", unsafe { self.value.s.as_str() }),
            PrimType::Array => {
                write!(f, "[")?;
                unsafe {
                    for (i, item) in self.value.a.1.iter().enumerate() {
                        match self.value.a.0 {
                            PrimType::Float => write!(f, "{:?}", item.f)?,
                            PrimType::Int => write!(f, "{:?}", item.i)?,
                            PrimType::Bool => write!(f, "{:?}", item.b)?,
                            PrimType::String => write!(f, "{:?}", item.s.as_str())?,
                            PrimType::Array => write!(f, "Sub Array")?,
                        }
                        if i != self.value.a.1.len() - 1 {
                            write!(f, ", ")?
                        }
                    }
                }
                write!(f, "]")
            }
        }
    }
}

pub fn val_from_literal(lit: Literal) -> Value {
    match lit {
        Literal::Float(f) => Value {
            t: PrimType::Float,
            value: ValueUnion { f },
        },
        Literal::Integer(i) => Value {
            t: PrimType::Int,
            value: ValueUnion { i },
        },
        Literal::Bool(b) => Value {
            t: PrimType::Bool,
            value: ValueUnion { b },
        },
        Literal::String(s) => Value {
            t: PrimType::String,
            value: ValueUnion { s },
        },
    }
}
