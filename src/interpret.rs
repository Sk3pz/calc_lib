use crate::{Definitions, Error, Number};
use crate::lex::{TokenType};
use crate::postfix::{ShuntedStack, ShuntedStackItem};

pub(crate) fn interpret(input: &mut ShuntedStack) -> Result<Number, Error> {
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
                TokenType::Num(n1) => {
                    match operand_1 {
                        TokenType::Num(n2) => {
                            // o1 is of type Number and o2 is of type Number
                            TokenType::Num(op.apply(n1.clone(), n2.clone())?)
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
        TokenType::Num(n) => Ok(n.clone()),
        _ => Err(Error::new_gen("Invalid expression"))
    }
}

pub(crate) fn interpret_with_definitions(input: &mut ShuntedStack, definitions: &Definitions) -> Result<Number, Error> {
    for x in 0..input.len() {
        let item = input.peek_at(x).unwrap();
        if item.is_operand() {
            let operand = item.get_operand().unwrap();
            match operand {
                TokenType::Identifier(ident) => {
                    let value = definitions.get(ident);
                    if value.is_none() {
                        return Err(Error::new_gen(format!("variable {} is not defined.", ident)));
                    }
                    input.replace(x, ShuntedStackItem::new_operand(TokenType::Num(value.unwrap().clone())));
                }
                _ => {}
            }
        }
    }
    interpret(input)
}