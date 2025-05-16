#![allow(dead_code)]
use crate::tokenizer::*;
use crate::value::Type;
use ordered_float::OrderedFloat;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    name: String,
}
impl Symbol {
    pub fn new(name: String) -> Symbol {
        Symbol { name }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Bool(bool),
}
impl Literal {
    pub fn get_type(&self) -> Type {
        match self {
            Literal::Integer(_) => Type::Int,
            Literal::Float(_) => Type::Float,
            Literal::String(_) => Type::String,
            Literal::Bool(_) => Type::Bool,
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Integer(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Lang(PreToken),
    Symb(Symbol),
    Lit(Literal),
}

#[derive(Debug, Clone)]
pub struct ParsingError {
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error on line {}:\n\t{}", self.line, self.message)
    }
}
impl Error for ParsingError {}

fn parse_literal(s: String) -> Result<Token, ParsingError> {
    if s.starts_with("'") && s.ends_with("'") {
        Ok(Token::Lit(Literal::String(s[1..s.len() - 1].to_owned())))
    } else if s.parse::<i64>().is_ok() {
        Ok(Token::Lit(Literal::Integer(s.parse().unwrap())))
    } else if s.parse::<f64>().is_ok() {
        Ok(Token::Lit(Literal::Float(OrderedFloat(s.parse().unwrap()))))
    } else if s == "true" || s == "false" {
        if s == "true" {
            Ok(Token::Lit(Literal::Bool(true)))
        } else {
            Ok(Token::Lit(Literal::Bool(false)))
        }
    } else {
        panic!("where is my literal??")
    }
}

fn parse_symbol(s: String) -> Result<Token, ParsingError> {
    Ok(Token::Symb(Symbol::new(s)))
}

fn parse_word(s: String) -> Result<Token, ParsingError> {
    if s.starts_with("\"")
        || s.starts_with("0")
        || s.starts_with("1")
        || s.starts_with("2")
        || s.starts_with("3")
        || s.starts_with("4")
        || s.starts_with("5")
        || s.starts_with("6")
        || s.starts_with("7")
        || s.starts_with("8")
        || s.starts_with("9")
        || s.starts_with(".")
        || s == "true"
        || s == "false"
    {
        parse_literal(s)
    } else {
        Ok(Token::Symb(Symbol::new(s)))
    }
}

fn collapse_array_types(tokens: Vec<PreTokenized>) -> Vec<PreTokenized> {
    let mut out: Vec<PreTokenized> = Vec::new();
    let mut i: usize = 0;
    let mut check = true;
    while i < tokens.len() {
        match tokens[i] {
            PreTokenized::T(PreToken::DEL(Delimeter::LBracket)) => {
                if !check {
                    out.push(tokens[i].clone());
                    i += 1;
                    continue;
                }
                let mut count: isize = 1;
                let mut max: usize = 1;
                let mut j = i + 1;
                while (count != 0) && j < tokens.len() {
                    match tokens[j] {
                        PreTokenized::T(PreToken::DEL(Delimeter::LBracket)) => {
                            count += 1;
                            max += 1;
                        }
                        PreTokenized::T(PreToken::DEL(Delimeter::RBracket)) => count -= 1,
                        PreTokenized::T(PreToken::TYPE(_)) => {}
                        _ => {
                            count = -1;
                            break;
                        }
                    }
                    j += 1;
                }
                if count == 0 && j == i + 2 {
                    out.push(tokens[i].clone());
                    i += 1;
                } else if count == 0 {
                    if let PreTokenized::T(PreToken::TYPE(t)) = &tokens[i + max] {
                        let mut arr_type = t.clone();
                        for _ in 0..max {
                            arr_type = Type::Array(Box::new(arr_type));
                        }
                        out.push(PreTokenized::T(PreToken::TYPE(arr_type)));
                        i = j;
                    } else {
                        out.push(tokens[i].clone());
                        i += 1;
                    }
                } else if count == -1 {
                    out.push(tokens[i].clone());
                    i += 1;
                } else {
                    panic!("unclosed array");
                }
            }
            PreTokenized::T(PreToken::EOL) => {
                check = true;
                out.push(tokens[i].clone());
                i += 1;
            }
            PreTokenized::T(PreToken::KW(Keyword::Kerchow)) => {
                check = false;
                out.push(tokens[i].clone());
                i += 1;
            }
            _ => {
                out.push(tokens[i].clone());
                i += 1;
            }
        }
    }
    out
}

pub fn parse_line(line: &str, out: &mut Vec<Token>) -> Result<(), ParsingError> {
    let pre_tokens = collapse_array_types(tokenize_line(line.to_string()));
    for token in pre_tokens.iter() {
        match token {
            PreTokenized::T(t) => out.push(Token::Lang(t.clone())),
            PreTokenized::S(s) => {
                out.push(parse_word(s.to_string())?);
            }
        }
    }
    Ok(())
}

pub fn parse(path: &str, out: &mut Vec<Token>) -> Result<(), Box<dyn Error>> {
    let mut scanner = Scanner::new();
    scanner.load_file(path)?;
    while let Some(line) = scanner.get_next_line() {
        parse_line(&line, out)?;
    }
    out.reverse();
    Ok(())
}
