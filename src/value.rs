use crate::parser::Literal;
use ordered_float::OrderedFloat;
use std::boxed::Box;
use std::boxed::ThinBox;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Clone)]
pub enum Type {
    Float,
    Int,
    Bool,
    String,
    Array(Box<Type>),
    AnyType,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Float, Type::Float) => true,
            (Type::Int, Type::Int) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::String, Type::String) => true,
            (Type::Array(x), Type::Array(y)) => *x == *y,
            (Type::AnyType, Type::Int) => true,
            (Type::AnyType, Type::Float) => true,
            (Type::AnyType, Type::Bool) => true,
            (Type::AnyType, Type::String) => true,
            (Type::Int, Type::AnyType) => true,
            (Type::Float, Type::AnyType) => true,
            (Type::Bool, Type::AnyType) => true,
            (Type::String, Type::AnyType) => true,
            _ => false,
        }
    }
}

impl Eq for Type {}

impl Type {
    pub fn array_depth(&self) -> u8 {
        match self {
            Type::Array(t) => 1 + t.deref().array_depth(),
            _ => 0,
        }
    }
}

pub union Value {
    pub f: OrderedFloat<f64>,
    pub i: i64,
    pub b: bool,
    pub a: ManuallyDrop<ThinBox<Vec<Value>>>,
    pub s: ManuallyDrop<ThinBox<String>>, // TODO: ThinBox<str>
}

impl Clone for Value {
    fn clone(&self) -> Self {
        unsafe { std::mem::transmute_copy(self) }
    }
}

impl Value {
    pub fn recursive_drop(mut self, depth: u8) {
        if depth != 0 {
            unsafe {
                for item in self.a.deref_mut().iter_mut() {
                    item.clone().recursive_drop(depth - 1);
                }
                ManuallyDrop::drop(&mut self.a);
            }
        }
    }
}

impl Value {
    pub fn fmt(&self, t: &Type, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match t {
            Type::Float => self.fmt_float(&self, f),
            Type::Int => self.fmt_int(&self, f),
            Type::Bool => self.fmt_bool(&self, f),
            Type::String => self.fmt_string(&self, f),
            Type::Array(sub_type) => self.fmt_array(&self, &sub_type.deref().clone(), f),
            Type::AnyType => panic!("AnyType"),
        }
    }
    fn fmt_float(&self, v: &Value, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { v.f })
    }
    fn fmt_int(&self, v: &Value, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { v.i })
    }
    fn fmt_bool(&self, v: &Value, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { v.b })
    }
    fn fmt_string(&self, v: &Value, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { v.s.deref() })
    }
    fn fmt_array(
        &self,
        v: &Value,
        sub_type: &Type,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for item in unsafe { v.a.deref().iter() } {
            match sub_type {
                Type::Float => self.fmt_float(item, f)?,
                Type::Int => self.fmt_int(item, f)?,
                Type::Bool => self.fmt_bool(item, f)?,
                Type::String => self.fmt_string(item, f)?,
                Type::Array(sub_sub_type) => {
                    self.fmt_array(item, &sub_sub_type.deref().clone(), f)?
                }
                Type::AnyType => panic!("AnyType"),
            }
        }
        Ok(())
    }
}

pub struct PrintValWrapper<'v> {
    pub val: &'v Value,
    pub t: &'v Type,
}

impl std::fmt::Display for PrintValWrapper<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.val.fmt(self.t, f)
    }
}

pub fn val_from_literal(lit: Literal) -> Value {
    match lit {
        Literal::Float(f) => Value { f },
        Literal::Integer(i) => Value { i },
        Literal::Bool(b) => Value { b },
        Literal::String(s) => Value {
            s: ManuallyDrop::new(ThinBox::new(s)),
        },
    }
}
