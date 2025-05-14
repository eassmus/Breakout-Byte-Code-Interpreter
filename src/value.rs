use crate::parser::Literal;
use ordered_float::OrderedFloat;
use std::boxed::ThinBox;
use std::mem::ManuallyDrop;
use std::ops::Deref;

#[derive(Debug)]
pub enum PrimType {
    Float,
    Int,
    Bool,
    String,
    Array(ThinBox<PrimType>),
}

pub union ValueUnion {
    pub f: OrderedFloat<f64>,
    pub i: i64,
    pub b: bool,
    pub a: ManuallyDrop<ThinBox<Vec<ValueUnion>>>,
    pub s: ManuallyDrop<ThinBox<String>>,
}

impl Clone for ValueUnion {
    fn clone(&self) -> Self {
        unsafe { std::mem::transmute_copy(self) }
    }
}

pub struct Value {
    pub value: ValueUnion,
}

impl Value {
    pub fn fmt(&self, t: &PrimType, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match t {
            PrimType::Float => self.fmt_float(&self.value, f),
            PrimType::Int => self.fmt_int(&self.value, f),
            PrimType::Bool => self.fmt_bool(&self.value, f),
            PrimType::String => self.fmt_string(&self.value, f),
            PrimType::Array(sub_type) => self.fmt_array(&self.value, sub_type, f),
        }
    }
    fn fmt_float(&self, v: &ValueUnion, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { v.f })
    }
    fn fmt_int(&self, v: &ValueUnion, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { v.i })
    }
    fn fmt_bool(&self, v: &ValueUnion, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { v.b })
    }
    fn fmt_string(&self, v: &ValueUnion, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { v.s.as_str() })
    }
    fn fmt_array(
        &self,
        v: &ValueUnion,
        sub_type: &PrimType,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for item in unsafe { v.a.deref().iter() } {
            match sub_type {
                PrimType::Float => self.fmt_float(&item, f)?,
                PrimType::Int => self.fmt_int(&item, f)?,
                PrimType::Bool => self.fmt_bool(&item, f)?,
                PrimType::String => self.fmt_string(&item, f)?,
                PrimType::Array(sub_type) => self.fmt_array(&item, sub_type, f)?,
            }
        }
        Ok(())
    }
}

pub fn val_from_literal(lit: Literal) -> Value {
    match lit {
        Literal::Float(f) => Value {
            value: ValueUnion { f },
        },
        Literal::Integer(i) => Value {
            value: ValueUnion { i },
        },
        Literal::Bool(b) => Value {
            value: ValueUnion { b },
        },
        Literal::String(s) => Value {
            value: ValueUnion {
                s: ManuallyDrop::new(ThinBox::new(s)),
            },
        },
    }
}
