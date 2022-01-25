use crate::{Definitions, Error, Functions};
use crate::lex::{Token};
use crate::postfix::{ShuntedStack, ShuntedStackItem};

pub(crate) fn interpret(input: &mut ShuntedStack) -> Result<f64, Error> {
    // loop through the stack until an operator is found, pushing the operands onto the operand stack
    // in the process
    let mut operand_stack = Vec::new();
    for item in input {
        if item.is_operand() {
            operand_stack.push(item.get_operand().unwrap().clone());
        } else {
            let op = item.get_operator().unwrap();
            if !op.can_apply() {
                return Err(Error::InvalidOperator { op: op.to_string() });
            }
            let operand_1 = operand_stack.pop().unwrap();
            let operand_2 = operand_stack.pop().unwrap();
            let r = match operand_2 {
                Token::Num(n1) => {
                    match operand_1 {
                        Token::Num(n2) => {
                            // o1 is of type Number and o2 is of type Number
                            Token::Num(op.apply(n1.clone(), n2.clone())?)
                        }
                        _ => return Err(Error::InvalidOperand { op: operand_1.to_string() }),
                    }
                }
                _ => return Err(Error::InvalidOperand { op: operand_1.to_string() })
            };
            operand_stack.push(r);
        }
    }

    if operand_stack.len() != 1 {
        return Err(Error::InvalidExpression { reason: "Invalid operand stack ending size".to_string() });
    }

    let result = operand_stack.pop().unwrap();
    match result {
        Token::Num(n) => Ok(n.clone()),
        _ => Err(Error::InvalidExpression { reason: format!("Invalid interpreted value: {}", result) })
    }
}

pub(crate) fn interpret_fn(ident: &String, args: &Vec<Token>, functions: &Functions, definitions: Option<&Definitions>) -> Result<f64, Error> {
    let value = functions.get(ident);
    if value.is_none() {
        return Err(Error::UndefinedFunction { name: ident.to_string() });
    }

    // replace args with numbers
    let mut pass_args = Vec::new();
    for a in args {
        if let Token::Num(n) = a {
            pass_args.push(n.clone());
        } else {
            match a {
                Token::Identifier(s) => {
                    if definitions.is_some() {
                        let value = definitions.unwrap().get(s);
                        if value.is_none() {
                            return Err(Error::UndefinedVariable { name: s.to_string() });
                        }
                        pass_args.push(value.unwrap().clone());
                    } else {
                        return Err(Error::InvalidArgument { name: ident.to_string(), value: a.to_string() });
                    }
                }
                Token::Function(i, a) => {
                    pass_args.push(interpret_fn(i, a, functions, definitions)?);
                }
                _ => {
                    return Err(Error::InvalidArgument { name: ident.to_string(), value: a.to_string() });
                }
            }
        }
    }

    value.unwrap()(pass_args)
}

pub(crate) fn interpret_with_definitions(input: &mut ShuntedStack, definitions: Option<&Definitions>, functions: Option<&Functions>) -> Result<f64, Error> {
    if definitions.is_some() {
        let definitions = definitions.unwrap();
        for x in 0..input.len() {
            let item = input.peek_at(x).unwrap();
            if item.is_operand() {
                let operand = item.get_operand().unwrap();
                match operand {
                    Token::Identifier(ident) => {
                        let value = definitions.get(ident);
                        if value.is_none() {
                            return Err(Error::UndefinedVariable { name: ident.to_string() });
                        }
                        input.replace(x, ShuntedStackItem::new_operand(Token::Num(value.unwrap().clone())));
                    }
                    _ => {}
                }
            }
        }
    }
    if functions.is_some() {
        let functions = functions.unwrap();
        for x in 0..input.len() {
            let item = input.peek_at(x).unwrap();
            if item.is_operand() {
                let operand = item.get_operand().unwrap();
                if let Token::Function(ident, args) = operand {
                    let val = interpret_fn(ident, args, functions, definitions)?;
                    input.replace(x, ShuntedStackItem::new_operand(Token::Num(val)));
                }
            }
        }
    }
    interpret(input)
}