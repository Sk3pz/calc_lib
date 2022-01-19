use std::fmt::{Display, Formatter};
use crate::input_reader::InputReader;
use crate::{Error, InputPos, Number};
use crate::operator::Operator;

#[derive(Debug, Clone)]
pub enum TokenType {
    Operator(Operator),
    Identifier(String),
    Num(Number),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TokenType::Operator(o) => write!(f, "Operator:{}", o),
            TokenType::Identifier(ref s) => write!(f, "Ident:{}", s),
            TokenType::Num(n) => write!(f, "Number:{}", n),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) pos: InputPos,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, pos: InputPos) -> Self {
        Token {
            token_type,
            pos,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.token_type)
    }
}

fn lex_ident(input: &mut InputReader) -> Result<Token, Error> {
    let mut ident = String::new();
    let start = input.pos();
    while let Some(c) = input.peek() {
        if c.is_alphanumeric() {
            ident.push(c);
            input.consume();
        } else {
            break;
        }
    }
    Ok(Token::new(TokenType::Identifier(ident), start))
}

fn lex_number(input: &mut InputReader) -> Result<Token, Error> {
    let mut number = String::new();
    let start = input.pos();
    let mut decimal = false;
    while let Some(c) = input.peek() {
        if c.is_numeric() {
            number.push(c);
            input.consume();
        } else if c == '.' {
            if decimal {
                return Err(Error::new(
                    "Invalid number",
                    start,
                ));
            }
            decimal = true;
            number.push(c);
            input.consume();
        } else {
            break;
        }
    }
    if decimal {
        let f = number.parse::<f64>();
        if f.is_err() {
            return Err(Error::new_gen(format!("Tried to parse an invalid number: {}", number)));
        }
        Ok(Token::new(TokenType::Num(Number::new(f.unwrap())), start))
    } else {
        let n = number.parse::<i128>();
        if n.is_err() {
            return Err(Error::new_gen(format!("Tried to parse an invalid number: {}", number)));
        }
        Ok(Token::new(TokenType::Num(Number::new(n.unwrap() as f64)), start))
    }
}

pub(crate) fn lex(input: &mut InputReader, allow_idents: bool) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    while let Some(c) = input.peek() {
        match c {
            '+' => {
                tokens.push(Token::new(TokenType::Operator(Operator::Add), input.pos()));
                input.consume();
            }
            '-' => {
                tokens.push(Token::new(TokenType::Operator(Operator::Sub), input.pos()));
                input.consume();
            }
            '*' => {
                tokens.push(Token::new(TokenType::Operator(Operator::Mul), input.pos()));
                input.consume();
            }
            '/' | '÷' => {
                tokens.push(Token::new(TokenType::Operator(Operator::Div), input.pos()));
                input.consume();
            }
            '%' => {
                tokens.push(Token::new(TokenType::Operator(Operator::Mod), input.pos()));
                input.consume();
            }
            '^' => {
                tokens.push(Token::new(TokenType::Operator(Operator::Pow), input.pos()));
                input.consume();
            }
            '=' => {
                tokens.push(Token::new(TokenType::Operator(Operator::Assign), input.pos()));
                input.consume();
            }
            '(' => {
                tokens.push(Token::new(TokenType::Operator(Operator::LeftParen), input.pos()));
                input.consume();
            }
            ')' => {
                tokens.push(Token::new(TokenType::Operator(Operator::RightParen), input.pos()));
                input.consume();
            }
            _ if (c.is_alphabetic() || c == '_') && allow_idents => tokens.push(lex_ident(input)?),
            _ if c.is_numeric() => tokens.push(lex_number(input)?),
            ' ' | '\t' | '\r' => {
                input.consume();
            }
            '\n' => {
                input.pos().line += 1;
                input.pos().ch = 0;
                input.consume();
            }
            _ => {
                return Err(Error::new(
                    format!("Unexpected character: {}", c),
                    input.pos(),
                ));
            }
        }
    }

    Ok(tokens)
}