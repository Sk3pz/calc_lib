use crate::{Definitions, Error};
use crate::lex::{TokenType};
use crate::postfix::{ShuntedStack, ShuntedStackItem};

pub(crate) fn interpret(input: &mut ShuntedStack) -> Result<i128, Error> {
    if input.is_empty() {
        return Err(Error::new_gen("Invalid expression: input is empty"));
    }
    // loop through the stack until an operator is found, pushing the operands onto the operand stack
    // in the process
    let mut operand_stack = Vec::new();
    for item in input {
        if item.is_operand() {
            operand_stack.push(item.get_operand().unwrap().clone());
        } else {
            let op = item.get_operator().unwrap();
            let operand_1 = operand_stack.pop().unwrap();
            let operand_2 = operand_stack.pop().unwrap();
            let r = match operand_2 {
                TokenType::Number(n1) => {
                    match operand_1 {
                        TokenType::Number(n2) => {
                            // o1 is of type Number and o2 is of type Number
                            TokenType::Number(op.apply(n1.clone(), n2.clone())?)
                        }
                        TokenType::Decimal(_) => {
                            return Err(Error::new_gen(
                                "Cannot perform arithmetic on decimal numbers in integer mode"
                            ));
                        }
                        _ => return Err(Error::new_gen("Invalid operand")),
                    }
                }
                TokenType::Decimal(_) =>
                    return Err(Error::new_gen(
                        "Cannot perform arithmetic on decimal numbers in integer mode")),
                _ => return Err(Error::new_gen("Invalid operand"))
            };
            operand_stack.push(r);
        }
    }

    if operand_stack.len() != 1 {
        return Err(Error::new_gen("Invalid expression"));
    }

    let result = operand_stack.pop().unwrap();
    match result {
        TokenType::Number(n) => Ok(n.clone()),
        TokenType::Decimal(_) => return Err(Error::new_gen(
            "Cannot perform arithmetic on decimal numbers in integer mode")),
        _ => Err(Error::new_gen("Invalid expression"))
    }
}

pub(crate) fn interpret_f64(input: &mut ShuntedStack) -> Result<f64, Error> {
    if input.is_empty() {
        return Err(Error::new_gen("Invalid expression: input is empty"));
    }
    // loop through the stack until an operator is found, pushing the operands onto the operand stack
    // in the process
    let mut operand_stack = Vec::new();
    for item in input {
        if item.is_operand() {
            operand_stack.push(item.get_operand().unwrap().clone());
        } else {
            let op = item.get_operator().unwrap();
            let operand_1 = operand_stack.pop().unwrap();
            let operand_2 = operand_stack.pop().unwrap();
            let r = match operand_2 {
                TokenType::Number(n1) => {
                    match operand_1 {
                        TokenType::Number(n2) => {
                            // o1 is of type Number and o2 is of type Number
                            TokenType::Number(op.apply(n1.clone(), n2.clone())?)
                        }
                        TokenType::Decimal(n2) => {
                            // o1 is of type Number and o2 is of type Decimal
                            let o1_f = n1.clone() as f64;
                            TokenType::Decimal(op.apply_f64(o1_f, n2.clone())?)
                        }
                        _ => return Err(Error::new_gen("Invalid operand")),
                    }
                }
                TokenType::Decimal(n1) => {
                    match operand_1 {
                        TokenType::Number(n2) => {
                            // o1 is of type Decimal and o2 is of type Number
                            let o2_f = n2.clone() as f64;
                            TokenType::Decimal(op.apply_f64(n1.clone(), o2_f)?)
                        }
                        TokenType::Decimal(n2) => {
                            // o1 is of type Decimal and o2 is of type Decimal
                            TokenType::Decimal(op.apply_f64(n1.clone(), n2.clone())?)
                        }
                        _ => return Err(Error::new_gen("Invalid operand")),
                    }
                }
                _ => return Err(Error::new_gen("Invalid operand"))
            };
            operand_stack.push(r);
        }
    }

    if operand_stack.len() != 1 {
        return Err(Error::new_gen("Invalid expression"));
    }

    let result = operand_stack.pop().unwrap();
    match result {
        TokenType::Number(n) => Ok(n.clone() as f64),
        TokenType::Decimal(n) => Ok(n),
        _ => Err(Error::new_gen("Invalid expression"))
    }
}

pub(crate) fn interpret_with_definitions(input: &mut ShuntedStack, definitions: &Definitions<i128>) -> Result<i128, Error> {
    for x in 0..input.len() {
        let item = input.peek_at(x).unwrap();
        if item.is_operand() {
            let operand = item.get_operand().unwrap();
            match operand {
                TokenType::Identifier(ident) => {
                    let value = definitions.get(ident);
                    if value.is_none() {
                        return Err(Error::new_gen("Undefined identifier"));
                    }
                    input.replace(x, ShuntedStackItem::new_operand(TokenType::Number(value.unwrap().clone())));
                }
                _ => {}
            }
        }
    }
    interpret(input)
}

pub(crate) fn interpret_with_definitions_f64(input: &mut ShuntedStack, definitions: &Definitions<f64>) -> Result<f64, Error> {
    // replace identifiers with their number values
    for x in 0..input.len() {
        let item = input.peek_at(x).unwrap();
        if item.is_operand() {
            let operand = item.get_operand().unwrap();
            match operand {
                TokenType::Identifier(ident) => {
                    let value = definitions.get(ident);
                    if value.is_none() {
                        return Err(Error::new_gen("Undefined identifier"));
                    }
                    input.replace(x, ShuntedStackItem::new_operand(TokenType::Decimal(value.unwrap().clone())));
                }
                _ => {}
            }
        }
    }
    // solve the new equation
    interpret_f64(input)
}
