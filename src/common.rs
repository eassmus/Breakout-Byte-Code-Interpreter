#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Return,
    Constant,

    Add,
    Subtract,
    Multiply,
    Divide,

    True,
    False,

    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    Not,

    LoadLocalVar,
    StackLoadLocalVar,
    FunctionCall,
}
