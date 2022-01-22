use std::fmt::{Display, Formatter};
use crate::{Error, Number};
use crate::lex::{Token, TokenType};
use crate::operator::Operator;

#[derive(Debug, Clone)]
pub(crate) struct ShuntedStackItem {
    operator: Option<Operator>,
    operand: Option<TokenType>,
}

impl ShuntedStackItem {
    pub(crate) fn new_operand(statement: TokenType) -> Self {
        Self {
            operator: None,
            operand: Some(statement),
        }
    }

    pub(crate) fn new_operator(operator: Operator) -> Self {
        Self {
            operator: Some(operator),
            operand: None,
        }
    }

    pub(crate) fn is_operator(&self) -> bool {
        self.operator.is_some()
    }

    pub(crate) fn is_operand(&self) -> bool {
        self.operand.is_some()
    }

    pub(crate) fn get_operator(&self) -> Option<&Operator> {
        self.operator.as_ref()
    }

    pub(crate) fn get_operand(&self) -> Option<&TokenType> {
        self.operand.as_ref()
    }
}

impl Display for ShuntedStackItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_operator() {
            write!(f, "{}", self.get_operator().unwrap())
        } else {
            write!(f, "{}", self.get_operand().unwrap())
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ShuntedStack {
    items: Vec<ShuntedStackItem>,
    current_iter: usize
}

impl ShuntedStack {
    pub(crate) fn new() -> Self {
        Self {
            items: Vec::new(),
            current_iter: 0
        }
    }

    pub(crate) fn push(&mut self, item: ShuntedStackItem) {
        self.items.push(item);
    }

    pub(crate) fn peek_at(&self, index: usize) -> Option<&ShuntedStackItem> {
        self.items.get(index)
    }

    pub(crate) fn replace(&mut self, index: usize, item: ShuntedStackItem) {
        self.items[index] = item;
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }
}

impl Display for ShuntedStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for item in self.items.iter() {
            result.push_str(&format!("{}", item));
        }
        write!(f, "{}", result)
    }
}

impl Iterator for ShuntedStack {
    type Item = ShuntedStackItem;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.items.get(self.current_iter).cloned();
        self.current_iter += 1;
        i
    }
}

pub(crate) fn shunting_yard(tokens: &mut Vec<Token>) -> Result<ShuntedStack, Error> {
    let mut postfix = ShuntedStack::new();
    let mut op_stack: Vec<Operator> = Vec::new();

    if tokens.is_empty() {
        return Err(Error::new_gen("Empty expression"));
    }

    let mut last_op: Option<Operator> = None;
    let mut negative = false;
    let mut last_was_ident = false;

    let first = tokens.get(0).unwrap();
    if let TokenType::Operator(op) = &first.token_type {
        match op {
            Operator::Sub => {
                negative = true;
            }
            Operator::LeftParen => {}
            _ => {
                return Err(Error::new(format!("Invalid leading operator: {}", op), first.pos.clone()));
            }
        }
    }

    for token in tokens {
        match &token.token_type {
            TokenType::Num(_) => {
                if last_was_ident {
                    return Err(Error::new("Invalid Expression: Two identifiers or numbers found in a row", token.pos.clone()));
                }
                let mut t = token.clone();
                if negative {
                    if let TokenType::Num(x) = token.token_type.clone() {
                        t = Token::new(
                            TokenType::Num(Number::new(-x.value)),
                            token.pos.clone()
                        );
                    }
                }
                postfix.push(ShuntedStackItem::new_operand(t.token_type));
                last_was_ident = true;
                last_op = None;
                negative = false;
            }
            TokenType::Identifier(_) => {
                if last_was_ident {
                    return Err(Error::new("Invalid Expression: Two identifiers or numbers found in a row", token.pos.clone()));
                }
                postfix.push(ShuntedStackItem::new_operand(token.token_type.clone()));
                last_op = None;
                last_was_ident = true;
                negative = false;
            }
            TokenType::Function(_, _) => {
                if last_was_ident {
                    return Err(Error::new("Invalid Expression: Two identifiers or numbers found in a row", token.pos.clone()));
                }
                postfix.push(ShuntedStackItem::new_operand(token.token_type.clone()));
                last_op = None;
                last_was_ident = true;
                negative = false;
            }
            TokenType::Operator(op) => {
                match op {
                    Operator::LeftParen => {
                        op_stack.push(op.clone());
                        if last_was_ident {
                            return Err(Error::new("Invalid Expression: missing operator", token.pos.clone()));
                        }
                        last_op = None;
                        last_was_ident = false;
                        negative = false;
                    }
                    Operator::RightParen => {
                        last_was_ident = false;
                        let mut found = false;
                        while let Some(op2) = op_stack.pop() {
                            if op2 == Operator::LeftParen {
                                found = true;
                                break;
                            }
                            postfix.push(ShuntedStackItem::new_operator(op2));
                        }

                        if !found {
                            return Err(Error::new("Found closing ')' without matching '('", token.pos.clone()));
                        }

                        last_op = Some(op.clone());
                        negative = false;
                    }
                    _ => {
                        // handle unary operators
                        if last_op.is_some() {
                            if *op == Operator::Sub {
                                negative = true;
                                last_was_ident = false;
                                continue;
                            } else if last_op.as_ref().unwrap().clone() != Operator::LeftParen
                                && last_op.as_ref().unwrap().clone() != Operator::RightParen {
                                return Err(Error::new(format!("Invalid operator: {}", op), token.pos.clone()));
                            }
                        }

                        last_was_ident = false;

                        // handle normal operators
                        while let Some(op2) = op_stack.last() {
                            if *op2 == Operator::LeftParen {
                                break;
                            }
                            if op2.precedence() <= op.precedence() {
                                break
                            }
                            postfix.push(ShuntedStackItem::new_operator(op_stack.pop().unwrap()));
                        }
                        op_stack.push(op.clone());
                        last_op = Some(op.clone());
                        negative = false;
                    }
                }
            }
        }
    }

    while let Some(op) = op_stack.pop() {
        if op == Operator::LeftParen {
            return Err(Error::new_gen("Found '(' without matching ')'"));
        }
        postfix.push(ShuntedStackItem::new_operator(op));
    }

    Ok(postfix)
}