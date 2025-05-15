use crate::chunk::Chunk;
use crate::common::OpCode;
use crate::parser::*;
use crate::tokenizer::PreToken;
use crate::tokenizer::*;
use crate::value::*;

#[inline]
pub fn get_next(token_stream: &[Token]) -> Option<Token> {
    token_stream.last().cloned()
}
#[inline]
pub fn get_sec_next(token_stream: &[Token]) -> Option<Token> {
    token_stream.get(token_stream.len() - 2).cloned()
}

fn consume_function_args(
    token_stream: &mut Vec<Token>,
    local_variables: &mut Vec<(String, Type)>,
) -> Result<(), String> {
    while get_next(token_stream) != Some(Token::Lang(PreToken::KW(Keyword::Kerchow))) {
        let name = match get_next(token_stream) {
            Some(Token::Symb(n)) => n.name(),
            _ => return Err("Expected argument name".to_string()),
        };
        token_stream.pop();
        if get_next(token_stream) != Some(Token::Lang(PreToken::DEL(Delimeter::Colon))) {
            return Err("Expected colon".to_string());
        }
        token_stream.pop();
        let t = match get_next(token_stream) {
            Some(Token::Lang(PreToken::TYPE(t))) => t,
            _ => return Err("Expected type".to_string()),
        };
        token_stream.pop();
        local_variables.push((name, t));
    }
    Ok(())
}

fn consume_eval(
    chunk: &mut Chunk,
    token_stream: &mut Vec<Token>,
    local_variables: &Vec<(String, Type)>,
    function_signatures: &Vec<(String, Vec<Type>, Type)>,
    constants: &mut Vec<Value>,
) -> Result<Type, String> {
    match get_next(token_stream) {
        Some(Token::Lit(l)) => {
            chunk.add_opcode(OpCode::Constant);
            chunk.add_byte(constants.len() as u8);
            constants.push(val_from_literal(l.clone()));

            token_stream.pop();
            Ok(l.get_type())
        }
        Some(Token::Lang(PreToken::DEL(Delimeter::LBracket))) => {
            token_stream.pop();
            let mut t: Option<Type> = None;
            let mut num: u8 = 0;
            while get_next(token_stream) != Some(Token::Lang(PreToken::DEL(Delimeter::RBracket))) {
                let in_type = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if t.is_none() {
                    t = Some(in_type);
                } else if t.clone().unwrap() != in_type {
                    return Err(format!(
                        "Type mismatch, expected {:?}, got {:?}",
                        t.clone().unwrap(),
                        in_type
                    ));
                }
                num += 1;
            }
            chunk.add_opcode(OpCode::ConstructArray);
            chunk.add_byte(num);
            token_stream.pop();
            if t.is_none() {
                t = Some(Type::AnyType);
            }
            Ok(Type::Array(Box::new(t.unwrap())))
        }
        Some(Token::Symb(s)) => {
            for (i, item) in local_variables.iter().enumerate() {
                if item.0 == s.name() {
                    chunk.add_opcode(OpCode::StackLoadLocalVar);
                    chunk.add_byte(i as u8);
                    token_stream.pop();
                    return Ok(local_variables[i].1.clone());
                }
            }
            for (i, item) in function_signatures.iter().enumerate() {
                if item.0 == s.name() {
                    token_stream.pop();
                    for j in 0..item.1.len() {
                        let t = consume_eval(
                            chunk,
                            token_stream,
                            local_variables,
                            function_signatures,
                            constants,
                        )?;
                        if t != function_signatures[i].1[j] {
                            return Err(format!(
                                "Type mismatch, expected {:?}, got {:?}",
                                function_signatures[i].1[j], t
                            ));
                        }
                    }
                    chunk.add_opcode(OpCode::FunctionCall);
                    chunk.add_byte(i as u8);
                    chunk.add_byte(item.1.len() as u8);
                    return Ok(function_signatures[i].2.clone());
                }
            }
            Err("Unknown symbol".to_string())
        }
        Some(Token::Lang(PreToken::OP(op))) => match op {
            Operator::Or => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != Type::Bool {
                    return Err(format!("Type mismatch, expected bool, got {type1}",));
                }
                if type2 != Type::Bool {
                    return Err(format!("Type mismatch, expected bool, got {type2}",));
                }
                chunk.add_opcode(OpCode::Or);
                Ok(Type::Bool)
            }
            Operator::And => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != Type::Bool {
                    return Err(format!("Type mismatch, expected bool, got {type1}",));
                }
                if type2 != Type::Bool {
                    return Err(format!("Type mismatch, expected bool, got {type2}",));
                }
                chunk.add_opcode(OpCode::And);
                Ok(Type::Bool)
            }
            Operator::Gt => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != type2 {
                    return Err(format!("Type mismatch, expected {type1}, got {type2}",));
                }
                if type1 == Type::Int {
                    chunk.add_opcode(OpCode::GreaterThanI);
                } else if type1 == Type::Float {
                    chunk.add_opcode(OpCode::GreaterThanF);
                } else {
                    return Err(format!("Type mismatch, expected {type1}, got {type2}",));
                }
                Ok(Type::Bool)
            }
            Operator::Lt => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != type2 {
                    return Err("Type mismatch".to_string());
                }
                if type1 == Type::Int {
                    chunk.add_opcode(OpCode::LessThanI);
                } else if type1 == Type::Float {
                    chunk.add_opcode(OpCode::LessThanF);
                } else {
                    return Err("Type mismatch".to_string());
                }
                Ok(Type::Bool)
            }
            Operator::Geq => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != type2 {
                    return Err("Type mismatch".to_string());
                }
                if type1 == Type::Int {
                    chunk.add_opcode(OpCode::GreaterThanOrEqualI);
                } else if type1 == Type::Float {
                    chunk.add_opcode(OpCode::GreaterThanOrEqualF);
                } else {
                    return Err("Type mismatch".to_string());
                }
                Ok(Type::Bool)
            }
            Operator::Leq => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != type2 {
                    return Err("Type mismatch".to_string());
                }
                if type1 == Type::Int {
                    chunk.add_opcode(OpCode::LessThanOrEqualI);
                } else if type1 == Type::Float {
                    chunk.add_opcode(OpCode::LessThanOrEqualF);
                } else {
                    return Err("Type mismatch".to_string());
                }
                Ok(Type::Bool)
            }
            Operator::Plus => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != type2 {
                    return Err("Type mismatch".to_string());
                }
                if type1 == Type::Int {
                    chunk.add_opcode(OpCode::AddI);
                } else if type1 == Type::Float {
                    chunk.add_opcode(OpCode::AddF);
                } else {
                    return Err("Type mismatch".to_string());
                }
                Ok(type1)
            }
            Operator::Minus => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != type2 {
                    return Err("Type mismatch".to_string());
                }
                if type1 == Type::Int {
                    chunk.add_opcode(OpCode::SubtractI);
                } else if type1 == Type::Float {
                    chunk.add_opcode(OpCode::SubtractF);
                } else {
                    return Err("Type mismatch".to_string());
                }
                Ok(type1)
            }
            Operator::Mult => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != type2 {
                    return Err("Type mismatch".to_string());
                }
                if type1 == Type::Int {
                    chunk.add_opcode(OpCode::MultiplyI);
                } else if type1 == Type::Float {
                    chunk.add_opcode(OpCode::MultiplyF);
                } else {
                    return Err("Type mismatch".to_string());
                }
                Ok(type1)
            }
            Operator::Div => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != type2 {
                    return Err("Type mismatch".to_string());
                }
                if type1 == Type::Int {
                    chunk.add_opcode(OpCode::DivideI);
                } else if type1 == Type::Float {
                    chunk.add_opcode(OpCode::DivideF);
                } else {
                    return Err("Type mismatch".to_string());
                }
                Ok(type1)
            }
            Operator::Mod => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != Type::Int || type2 != Type::Int {
                    return Err(format!(
                        "Type mismatch, expected ints, got {type1}, {type2}",
                    ));
                }
                chunk.add_opcode(OpCode::Mod);
                Ok(Type::Int)
            }
            Operator::Cond => {
                token_stream.pop();
                let cond_type = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if cond_type != Type::Bool {
                    return Err("Type mismatch".to_string());
                }
                let mut chunk1 = Chunk::new(Vec::new());
                let type1 = consume_eval(
                    &mut chunk1,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let eval1len = chunk1.get_length() as u8;
                let mut chunk2 = Chunk::new(Vec::new());
                let type2 = consume_eval(
                    &mut chunk2,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let eval2len = chunk2.get_length() as u8;
                if type1 != type2 {
                    return Err("Type mismatch".to_string());
                }
                chunk.add_opcode(OpCode::AdvanceIfFalse);
                chunk.add_byte(eval1len + 2);
                chunk.add_chunk(&mut chunk1);
                drop(chunk1);
                chunk.add_opcode(OpCode::Advance);
                chunk.add_byte(eval2len);
                chunk.add_chunk(&mut chunk2);
                drop(chunk2);
                Ok(type1)
            }
            Operator::Eq => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 != type2 {
                    return Err(format!("Type mismatch, got {type1} and {type2}",));
                }
                match type1 {
                    Type::Int => chunk.add_opcode(OpCode::EqualI),
                    Type::Float => chunk.add_opcode(OpCode::EqualF),
                    Type::Bool => chunk.add_opcode(OpCode::EqualB),
                    Type::String => chunk.add_opcode(OpCode::EqualS),
                    _ => return Err("Type mismatch".to_string()),
                }
                Ok(Type::Bool)
            }
            Operator::Concat => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if type1 == Type::String && type2 == Type::String {
                    chunk.add_opcode(OpCode::ConcatStr);
                    Ok(Type::String)
                } else if let Type::Array(_) = type1
                    && type1 == type2
                {
                    chunk.add_opcode(OpCode::ConcatArr);
                    Ok(type1)
                } else {
                    return Err("Type mismatch".to_string());
                }
            }
            Operator::Index => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                let type2 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if let Type::Array(t) = type1
                    && type2 == Type::Int
                {
                    chunk.add_opcode(OpCode::Index);
                    Ok(*t)
                } else {
                    return Err("Type mismatch".to_string());
                }
            }
            Operator::Length => {
                token_stream.pop();
                let type1 = consume_eval(
                    chunk,
                    token_stream,
                    local_variables,
                    function_signatures,
                    constants,
                )?;
                if let Type::Array(_) = type1 {
                    chunk.add_opcode(OpCode::LenArr);
                    Ok(Type::Int)
                } else if let Type::String = type1 {
                    chunk.add_opcode(OpCode::LenStr);
                    Ok(Type::Int)
                } else {
                    return Err("Type mismatch".to_string());
                }
            }
            _ => todo!(),
        },
        _ => Err("Expected expression".to_string()),
    }
}

fn consume_def(
    token_stream: &mut Vec<Token>,
    func_signatures: &mut Vec<(String, Vec<Type>, Type)>,
    constants: &mut Vec<Value>,
) -> Result<(Option<Chunk>, bool), String> {
    let mut chunk = Chunk::new(Vec::new());
    let mut is_main = false;

    let mut local_variables: Vec<(String, Type)> = Vec::new();

    // consume name and fluf symbols

    if get_next(token_stream) == Some(Token::Lang(PreToken::EOL)) {
        token_stream.pop();
        return Ok((None, false));
    }

    let t = match get_next(token_stream) {
        Some(Token::Lang(PreToken::TYPE(t))) => t,
        _ => {
            return Err(format!(
                "Expected type, got {:?}",
                get_next(token_stream).unwrap()
            ));
        }
    };
    token_stream.pop();
    let func_name = match get_next(token_stream) {
        Some(Token::Symb(n)) => n.name(),
        _ => {
            return Err(format!(
                "Expected function name, got {:?}",
                get_next(token_stream)
            ));
        }
    };
    token_stream.pop();
    if func_name == "main" {
        is_main = true;
    }
    if get_next(token_stream) != Some(Token::Lang(PreToken::KW(Keyword::Define))) {
        return Err("Expected define".to_string());
    }
    token_stream.pop();

    match get_sec_next(token_stream) {
        Some(Token::Lang(PreToken::DEL(Delimeter::Colon))) => {
            // add local variables
            consume_function_args(token_stream, &mut local_variables)?;

            if get_next(token_stream) != Some(Token::Lang(PreToken::KW(Keyword::Kerchow))) {
                return Err("Expected kerchow".to_string());
            }
            token_stream.pop();

            func_signatures.push((
                func_name.clone(),
                local_variables.iter().map(|(_, lvt)| lvt.clone()).collect(),
                t.clone(),
            ));

            let eval_type = consume_eval(
                &mut chunk,
                token_stream,
                &local_variables,
                func_signatures,
                constants,
            )?;
            if t != eval_type {
                return Err(format!("Type mismatch, expected {t} got {eval_type}",));
            }
        }
        _ => {
            func_signatures.push((func_name.clone(), Vec::new(), t.clone()));
            let eval_type = consume_eval(
                &mut chunk,
                token_stream,
                &local_variables,
                func_signatures,
                constants,
            )?;
            if t != eval_type {
                return Err(format!("Type mismatch, expected {t} got {eval_type}",));
            }
        }
    }

    chunk.add_opcode(OpCode::Return);
    Ok((Some(chunk), is_main))
}

pub fn compile(
    token_stream: &mut Vec<Token>,
    func_signatures: &mut Vec<(String, Vec<Type>, Type)>,
    constants: &mut Vec<Value>,
) -> Result<(Vec<Chunk>, Option<usize>, Option<Type>), String> {
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut main: Option<usize> = None;
    let mut main_type: Option<Type> = None;
    let mut i = 0;
    while !token_stream.is_empty() {
        let (chunk, is_main) = consume_def(token_stream, func_signatures, constants)?;
        if chunk.is_none() {
            continue;
        }
        let chunk = chunk.unwrap();
        chunks.push(chunk);
        if is_main {
            main = Some(i);
            main_type = Some(func_signatures[i].2.clone());
        }
        i += 1;
    }
    Ok((chunks, main, main_type))
}
