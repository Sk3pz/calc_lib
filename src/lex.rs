use std::fmt::{Display, Formatter};
use crate::input_reader::InputReader;
use crate::Error;
use crate::operator::Operator;

#[derive(Debug, Clone)]
pub(crate) enum Token {
    Operator(Operator),
    Identifier(String),
    Num(f64),
    Function(String, Vec<Token>),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Token::Operator(o) => write!(f, "Operator:{}", o),
            Token::Identifier(ref s) => write!(f, "Ident:{}", s),
            Token::Num(n) => write!(f, "Number:{}", n),
            Token::Function(s, _) => write!(f, "Function:{}(...)", s),
        }
    }
}

fn lex_ident(input: &mut InputReader, allow_idents: bool) -> Result<Token, Error> {
    let mut ident = String::new();
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

                params.push(next_token(input, allow_idents)?);
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
                        return Err(Error::Expected { expected: ", or )".to_string(), found: c2.to_string()});
                    }
                }
            }

            return Ok(Token::Function(ident, params));
        } else {
            break;
        }
    }
    Ok(Token::Identifier(ident))
}

fn lex_number(input: &mut InputReader) -> Result<Token, Error> {
    let mut number = String::new();
    let mut decimal = false;
    while let Some(c) = input.peek() {
        if c.is_numeric() {
            number.push(c);
            input.consume();
        } else if c == '.' {
            if decimal {
                return Err(Error::InvalidNumber { found: number });
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
            return Err(Error::InvalidNumber { found: number });
        }
        Ok(Token::Num(f.unwrap()))
    } else {
        let n = number.parse::<i128>();
        if n.is_err() {
            return Err(Error::InvalidNumber { found: number });
        }
        Ok(Token::Num(n.unwrap() as f64))
    }
}

pub(crate) fn next_token(input: &mut InputReader, allow_idents: bool) -> Result<Token, Error> {
    let next = input.peek();
    if next.is_none() {
        return Err(Error::UnexpectedEOI);
    }
    let c = next.unwrap();
    Ok(match input.peek().unwrap() {
        '+' => {
            input.consume();
            Token::Operator(Operator::Add)
        }
        '-' => {
            input.consume();
            Token::Operator(Operator::Sub)
        }
        '*' => {
            input.consume();
            Token::Operator(Operator::Mul)
        }
        '/' | '÷' => {
            input.consume();
            Token::Operator(Operator::Div)
        }
        '%' => {
            input.consume();
            Token::Operator(Operator::Mod)
        }
        '^' => {
            input.consume();
            Token::Operator(Operator::Pow)
        }
        '=' => {
            input.consume();
            Token::Operator(Operator::Assign)
        }
        '(' => {
            input.consume();
            Token::Operator(Operator::LeftParen)
        }
        ')' => {
            input.consume();
            Token::Operator(Operator::RightParen)
        }
        _ if (c.is_alphabetic() || c == '_') && allow_idents => lex_ident(input, allow_idents)?,
        _ if c.is_numeric() => lex_number(input)?,
        _ => {
            return Err(Error::InvalidCharacter { c });
        }
    })
}

pub(crate) fn lex(input: &mut InputReader, allow_idents: bool) -> Result<Vec<Token>, Error> {
    if input.is_empty() {
        return Ok(vec![Token::Num(0.0)]);
    }

    let mut tokens = Vec::new();
    while let Some(c) = input.peek() {
        match c {
            ' ' | '\n' | '\t' | '\r' => {
                input.consume();
            }
            _ => tokens.push(next_token(input, allow_idents)?),
        }
    }

    Ok(tokens)
}