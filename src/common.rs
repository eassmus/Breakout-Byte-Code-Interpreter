#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Return,
    Constant,

    AddI,
    SubtractI,
    MultiplyI,
    DivideI,

    AddF,
    SubtractF,
    MultiplyF,
    DivideF,

    True,
    False,

    EqualI,
    EqualF,
    EqualS,
    EqualB,

    GreaterThanI,
    LessThanI,
    GreaterThanOrEqualI,
    LessThanOrEqualI,

    GreaterThanF,
    LessThanF,
    GreaterThanOrEqualF,
    LessThanOrEqualF,

    Advance,
    AdvanceIfFalse,

    Not,

    StackLoadLocalVar,
    FunctionCall,

    ConstructArray,

    ConcatArr,
    ConcatStr,

    LenArr,
    LenStr,

    DropArr,
    DropStr,

    Index,

    And,
    Or,

    Mod,

    NullCode,
}
impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Return,
            1 => OpCode::Constant,

            2 => OpCode::AddI,
            3 => OpCode::SubtractI,
            4 => OpCode::MultiplyI,
            5 => OpCode::DivideI,

            6 => OpCode::AddF,
            7 => OpCode::SubtractF,
            8 => OpCode::MultiplyF,
            9 => OpCode::DivideF,

            10 => OpCode::True,
            11 => OpCode::False,

            12 => OpCode::EqualI,
            13 => OpCode::EqualF,
            14 => OpCode::EqualS,
            15 => OpCode::EqualB,

            16 => OpCode::GreaterThanI,
            17 => OpCode::LessThanI,
            18 => OpCode::GreaterThanOrEqualI,
            19 => OpCode::LessThanOrEqualI,

            20 => OpCode::GreaterThanF,
            21 => OpCode::LessThanF,
            22 => OpCode::GreaterThanOrEqualF,
            23 => OpCode::LessThanOrEqualF,

            24 => OpCode::Advance,
            25 => OpCode::AdvanceIfFalse,

            26 => OpCode::Not,

            27 => OpCode::StackLoadLocalVar,
            28 => OpCode::FunctionCall,

            29 => OpCode::ConstructArray,

            30 => OpCode::ConcatArr,
            31 => OpCode::ConcatStr,

            32 => OpCode::LenArr,
            33 => OpCode::LenStr,

            34 => OpCode::DropArr,
            35 => OpCode::DropStr,

            36 => OpCode::Index,

            37 => OpCode::And,
            38 => OpCode::Or,

            39 => OpCode::Mod,

            _ => OpCode::NullCode,
        }
    }
}
