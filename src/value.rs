use crate::parser::Literal;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Copy)]
pub enum PrimType {
    Float,
    Int,
    Bool,
    Char,
}

#[derive(Clone, Copy)]
pub union ValueUnion {
    pub f: OrderedFloat<f64>,
    pub i: i64,
    pub b: bool,
    pub c: char,
}

#[derive(Clone, Copy)]
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
            PrimType::Char => write!(f, "{}", unsafe { self.value.c }),
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
        Literal::Char(c) => Value {
            t: PrimType::Char,
            value: ValueUnion { c },
        },
    }
}
