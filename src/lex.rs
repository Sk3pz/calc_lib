use std::fmt::{Display, Formatter};
use crate::input_reader::InputReader;
use crate::{Error, ErrorType, InputPos, Number};
use crate::operator::Operator;

#[derive(Debug, Clone)]
pub(crate) enum TokenType {
    Operator(Operator),
    Identifier(String),
    Num(Number),
    Function(String, Vec<TokenType>),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TokenType::Operator(o) => write!(f, "Operator:{}", o),
            TokenType::Identifier(ref s) => write!(f, "Ident:{}", s),
            TokenType::Num(n) => write!(f, "Number:{}", n),
            TokenType::Function(s, _) => write!(f, "Function:{}(...)", s),
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

fn lex_ident(input: &mut InputReader, allow_idents: bool) -> Result<Token, Error> {
    let mut ident = String::new();
    let start = input.pos();
    while let Some(c) = input.peek() {
        if c.is_alphanumeric() {
            ident.push(c);
            input.consume();
        } else if c == '(' {
            input.consume();
            let mut params = Vec::new();
            while let Some(c) = input.peek() {
                if c == ' ' || c == '\n' || c == '\t' || c == '\r' {
                    input.consume();
                    continue;
                }
                if c == ')' {
                    input.consume();
                    break;
                }

                params.push(next_token(input, allow_idents)?.token_type);
                while let Some(c2) = input.peek() {
                    if c2 == ' ' || c == '\n' || c == '\t' || c == '\r' {
                        input.consume();
                        continue;
                    }
                    if c2 == ')' {
                        break;
                    }
                    if c2 == ',' {
                        input.consume();
                        break;
                    } else {
                        return Err(Error::new(ErrorType::Expected { expected: ", or )".to_string(), found: c2.to_string()}, input.pos()));
                    }
                }
            }

            return Ok(Token::new(TokenType::Function(ident, params), start));
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
                    ErrorType::InvalidNumber { found: number },
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
            return Err(Error::new_gen(ErrorType::InvalidNumber { found: number }));
        }
        Ok(Token::new(TokenType::Num(Number::new(f.unwrap())), start))
    } else {
        let n = number.parse::<i128>();
        if n.is_err() {
            return Err(Error::new_gen(ErrorType::InvalidNumber { found: number }));
        }
        Ok(Token::new(TokenType::Num(Number::new(n.unwrap() as f64)), start))
    }
}

pub(crate) fn next_token(input: &mut InputReader, allow_idents: bool) -> Result<Token, Error> {
    let next = input.peek();
    if next.is_none() {
        return Err(Error::new_gen(ErrorType::UnexpectedEOI));
    }
    let c = next.unwrap();
    Ok(match input.peek().unwrap() {
        '+' => {
            input.consume();
            Token::new(TokenType::Operator(Operator::Add), input.pos())
        }
        '-' => {
            input.consume();
            Token::new(TokenType::Operator(Operator::Sub), input.pos())
        }
        '*' => {
            input.consume();
            Token::new(TokenType::Operator(Operator::Mul), input.pos())
        }
        '/' | '÷' => {
            input.consume();
            Token::new(TokenType::Operator(Operator::Div), input.pos())
        }
        '%' => {
            input.consume();
            Token::new(TokenType::Operator(Operator::Mod), input.pos())
        }
        '^' => {
            input.consume();
            Token::new(TokenType::Operator(Operator::Pow), input.pos())
        }
        '=' => {
            input.consume();
            Token::new(TokenType::Operator(Operator::Assign), input.pos())
        }
        '(' => {
            input.consume();
            Token::new(TokenType::Operator(Operator::LeftParen), input.pos())
        }
        ')' => {
            input.consume();
            Token::new(TokenType::Operator(Operator::RightParen), input.pos())
        }
        _ if (c.is_alphabetic() || c == '_') && allow_idents => lex_ident(input, allow_idents)?,
        _ if c.is_numeric() => lex_number(input)?,
        _ => {
            return Err(Error::new(
                ErrorType::InvalidCharacter { c },
                input.pos(),
            ));
        }
    })
}

pub(crate) fn lex(input: &mut InputReader, allow_idents: bool) -> Result<Vec<Token>, Error> {
    if input.is_empty() {
        return Ok(vec![Token::new(TokenType::Num(Number::new(0.0)), input.pos())]);
    }

    let mut tokens = Vec::new();
    while let Some(c) = input.peek() {
        match c {
            ' ' | '\t' | '\r' => {
                input.consume();
            }
            '\n' => {
                input.pos().line += 1;
                input.pos().ch = 0;
                input.consume();
            }
            _ => tokens.push(next_token(input, allow_idents)?),
        }
    }

    Ok(tokens)
}