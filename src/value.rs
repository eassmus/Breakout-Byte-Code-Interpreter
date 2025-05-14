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

impl std::fmt::Debug for ValueUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", unsafe { self.i })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Value {
    pub t: PrimType,
    pub value: ValueUnion,
}

pub fn val_from_literal(lit: &Literal) -> Value {
    match lit {
        Literal::Float(f) => Value {
            t: PrimType::Float,
            value: ValueUnion { f: *f },
        },
        Literal::Integer(i) => Value {
            t: PrimType::Int,
            value: ValueUnion { i: *i },
        },
        Literal::Bool(b) => Value {
            t: PrimType::Bool,
            value: ValueUnion { b: *b },
        },
        Literal::Char(c) => Value {
            t: PrimType::Char,
            value: ValueUnion { c: *c },
        },
    }
}
