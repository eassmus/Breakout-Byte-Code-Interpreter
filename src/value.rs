use ordered_float::OrderedFloat;

#[derive(Clone, Copy)]
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
