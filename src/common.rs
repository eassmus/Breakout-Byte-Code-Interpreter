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

    Advance,
    AdvanceIfFalse,

    Not,

    StackLoadLocalVar,
    FunctionCall,
}
impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            1 => OpCode::Constant,
            2 => OpCode::Add,
            3 => OpCode::Subtract,
            4 => OpCode::Multiply,
            5 => OpCode::Divide,
            6 => OpCode::True,
            7 => OpCode::False,
            8 => OpCode::Equal,
            9 => OpCode::NotEqual,
            10 => OpCode::GreaterThan,
            11 => OpCode::LessThan,
            12 => OpCode::GreaterThanOrEqual,
            13 => OpCode::LessThanOrEqual,
            14 => OpCode::Advance,
            15 => OpCode::AdvanceIfFalse,
            16 => OpCode::Not,
            17 => OpCode::StackLoadLocalVar,
            18 => OpCode::FunctionCall,
            _ => OpCode::Not,
        }
    }
}
