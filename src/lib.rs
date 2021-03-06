use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::input_reader::InputReader;
use crate::interpret::{interpret, interpret_with_definitions};

pub(crate) mod lex;
pub(crate) mod input_reader;
pub(crate) mod postfix;
pub(crate) mod interpret;
pub(crate) mod operator;

/// An enum representing an error that occurred
/// This allows for user handling of errors while still allowing them to just be
/// printed out if custom handling of errors is not needed.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    /// An error in which the input attempts a division by zero, which is undefined.
    DivByZero,
    /// An error in which the input attempts to raise a number to a negative power, which is undefined.
    NegativeExponent,
    /// An error in which the input contains an invalid character that can not be parsed.
    InvalidCharacter {
        /// The invalid character
        c: char
    },
    /// An error in which the input contains an invalid number (i.e. 2 decimal points).
    InvalidNumber {
        /// The number in string form that couldn't be parsed
        found: String
    },
    /// An error in which the parser expected something but got something else (invalid input).
    Expected {
        ///The expected input
        expected: String,
        /// What was found
        found: String
    },
    /// when the parser reaches an unexpected End Of Input when it is still expecting more input.
    UnexpectedEOI,
    /// When the interpreter finds an invalid operand, such as a variable identifier when none are defined.
    InvalidOperand {
        /// The invalid operand
        op: String
    },
    /// When the interpreter finds an invalid operator, such as a '=' while solving.
    InvalidOperator {
        /// The invalid operator
        op: String
    },
    /// When the interpreter has an invalid output, such as multiple values or a remaining operator.
    InvalidExpression {
        /// The reason for the error
        reason: String
    },
    /// When interpreting with definitions finds an undefined variable.
    UndefinedVariable {
        /// The name of the undefined variable
        name: String
    },
    /// When interpreting with definitions finds an undefined function.
    UndefinedFunction {
        /// The name of the undefined function
        name: String
    },
    /// When a function is called with the wrong number of arguments.
    InvalidArgumentCount {
        /// The name of the function
        name: String,
        /// The expected number of arguments
        expected: usize,
        /// The number of arguments received
        got: usize
    },
    /// When something other than a number or variable is passed to a function.
    InvalidArgument {
        /// The name of the function
        name: String,
        /// The invalid value that was passed
        value: String
    },
    /// When the interpreter finds an operator after another operator that should not be there.
    InvalidLeadingOperator {
        /// the operator that was found
        op: String
    },
    /// When the interpreter expects an operator (i.e. after a number) but gets something else.
    MissingOperator,
    /// When there is an incomplete pair of parentheses (i.e. open with no close or vice versa).
    MismatchedParentheses {
        /// The parenthesis that was found ('(' or ')')
        found: char,
        /// The parenthesis that was missing ('(' or ')')
        missing: char
    },
    /// Custom error messages.
    /// contains a String of the error message.
    /// this is not used by this program and is only used for custom error messages by the user
    Other(String),
}

impl Error {
    pub fn arg_count<S: Into<String>>(name: S, expected: usize, got: usize) -> Error {
        Error::InvalidArgumentCount {
            name: name.into(),
            expected,
            got
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DivByZero => write!(f, "Can't divide by zero"),
            Error::NegativeExponent => write!(f, "Can't raise a value to a negative power"),
            Error::InvalidCharacter { c } => write!(f, "Invalid character: {}", c),
            Error::InvalidNumber { found } => write!(f, "Invalid number: {}", found),
            Error::Expected { expected, found } => write!(f, "Expected '{}', found '{}'", expected, found),
            Error::UnexpectedEOI => write!(f, "Unexpected end of input"),
            Error::InvalidOperand { op } => write!(f, "Invalid operand: {}", op),
            Error::InvalidOperator { op } => write!(f, "Invalid operator: {}", op),
            Error::InvalidExpression { reason } => write!(f, "Invalid expression: {}", reason),
            Error::UndefinedVariable { name } => write!(f, "Undefined variable: {}", name),
            Error::UndefinedFunction { name } => write!(f, "Undefined function: {}", name),
            Error::InvalidArgumentCount { name, expected, got } => write!(f, "Invalid argument count for function '{}': expected {}, got {}", name, expected, got),
            Error::InvalidArgument { name, value } => write!(f, "Invalid argument for function '{}': {}", name, value),
            Error::InvalidLeadingOperator { op } => write!(f, "Invalid leading operator: {}", op),
            Error::MissingOperator => write!(f, "Missing operator"),
            Error::MismatchedParentheses { found, missing } => write!(f, "Mismatched parentheses: found '{}', missing '{}'", found, missing),
            Error::Other(s) => write!(f, "{}", s),
        }
    }
}

/// A list of definitions to pass into the crate to be used in the interpreter.
pub struct Definitions {
    pub(crate) map: HashMap<String, f64>,
}

impl Definitions {
    /// Create a new definition map
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// register a new definition to the map
    pub fn register<S: Into<String>, N: Into<f64>>(&mut self, name: S, value: N) {
        self.map.insert(name.into(), value.into());
    }

    pub fn exists<S: Into<String>>(&self, ident: S) -> bool {
        self.map.contains_key(ident.into().as_str())
    }

    /// Get a definition from the map
    pub(crate) fn get<S: Into<String>>(&self, ident: S) -> Option<&f64> {
        self.map.get(ident.into().as_str())
    }
}

/// A list of definitions of functions to pass into the interpreter to solve for the variables.
pub struct Functions<'a> {
    pub(crate) functions: HashMap<String, Box<dyn Fn(Vec<f64>) -> Result<f64, Error> + 'a>>,
}

impl<'a> Functions<'a> {
    /// Create a new list of functions
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    /// register a function
    pub fn register<S: Into<String>, F: Fn(Vec<f64>) -> Result<f64, Error> + 'a + Copy>(&mut self, name: S, f: F) {
        self.functions.insert(name.into(), Box::new(f));
    }

    /// check if a function exists
    pub fn exists<S: Into<String>>(&self, ident: S) -> bool {
        self.functions.contains_key(ident.into().as_str())
    }

    pub(crate) fn get<S: Into<String>>(&self, ident: S) -> Option<&Box<dyn Fn(Vec<f64>) -> Result<f64, Error> + 'a>> {
        let ident = ident.into();
        if !self.functions.contains_key(&ident) {
            return None;
        }
        self.functions.get(&ident)
    }
}

impl Default for Functions<'_> {
    /// create a new list of functions with the default functions:
    /// `log`, `sqrt`, `sin`, `cos`, `tan`
    fn default() -> Self {
        let mut funcs = Functions::new();
        funcs.register("log", |args| {
            if args.len() != 2 {
                return Err(Error::arg_count("log", 2, args.len()));
            }
            Ok(args[1].log(args[0]))
        });

        funcs.register("sqrt", |args| {
            if args.len() != 1 {
                return Err(Error::arg_count("sqrt", 1, args.len()));
            }
            Ok(args[0].sqrt())
        });

        funcs.register("sin", |args| {
            if args.len() != 1 {
                return Err(Error::arg_count("sin", 1, args.len()));
            }
            Ok(args[0].sin())
        });

        funcs.register("cos", |args| {
            if args.len() != 1 {
                return Err(Error::arg_count("cos", 1, args.len()));
            }
            Ok(args[0].cos())
        });

        funcs.register("tan", |args| {
            if args.len() != 1 {
                return Err(Error::arg_count("tan", 1, args.len()));
            }
            Ok(args[0].tan())
        });

        funcs
    }
}

/// Evaluates an equation in infix notation using the shunting yard algorithm.
/// This function does not accept defined variables or functions. See `evaluate_with_defined`.
/// # Usage Example:
/// ```
/// use calc_lib::evaluate;
///
/// let eval = evaluate("(1 + 2) * 3");
/// if eval.is_err() {
///     panic!("{}", eval.unwrap_err());
/// }
/// assert_eq!(eval.unwrap() as i64, 9);
/// ```
pub fn evaluate<S: Into<String>>(input: S) -> Result<f64, Error> {
    let mut input = InputReader::new(input.into());
    let mut tokens = lex::lex(&mut input, false)?;
    let mut shunted = postfix::shunting_yard(&mut tokens)?;
    interpret(&mut shunted)
}

/// Evaluates an expression in infix notation using the shunting yard algorithm.
/// this function takes the expression, a Definitions struct and a Functions struct which
/// allow for variables and functions to be interpreted within the expression.
///
/// # Usage Example:
///```
/// use calc_lib::{Definitions, evaluate_with_defined};
///
/// // define the variable 'x' as 3 for use in the expression
/// let mut defs = Definitions::new();
/// defs.register("x", 3);
///
/// let solved = evaluate_with_defined("(x + 3) / 3", Some(&defs), None);
/// assert_eq!(solved.unwrap() as i64, 2);
/// ```
///
/// # Usage with functions:
/// ```
/// use calc_lib::{Definitions, Functions, evaluate_with_defined, Error};
///
/// // define the variable 'x' as 3 for use in the expression
/// let mut defs = Definitions::new();
/// defs.register("x", 3);
///
/// // define the functions that can be used and their logic
/// let mut funcs = Functions::new();
/// // this shows the definition of the log function,
/// // which is already implemented in `Functions::default();`
/// funcs.register("log", |args| {
///     if args.len() != 2 {
///         return Err(Error::arg_count("log", 2, args.len()));
///     }
///     Ok(args[1].log(args[0]))
///});
///
/// let eval = evaluate_with_defined("log(2, 16)", Some(&defs), Some(&funcs));
/// assert_eq!(eval.unwrap() as i64, 4);
/// ```
pub fn evaluate_with_defined<S: Into<String>>(input: S, definitions: Option<&Definitions>, functions: Option<&Functions>) -> Result<f64, Error> {
    let mut input = InputReader::new(input.into());
    let mut tokens = lex::lex(&mut input, definitions.is_some() || functions.is_some())?;
    let mut shunted = postfix::shunting_yard(&mut tokens)?;
    interpret_with_definitions(&mut shunted, definitions, functions)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let expression = "(2 + 1) - 50 * 12 / 18 - (3 + 1) * 5";

        // (2 + 1) - 50 * 12 / 18 - (3 + 1) * 5
        // 3 - 33.333333 - 4 * 5
        // 3 - 33.333333 - 20
        // 3 - 53.333333
        // 50.333333

        let eval = evaluate(expression);
        if eval.is_err() {
            panic!("Encountered an error evaluating: {}", eval.unwrap_err());
        }
        println!("{}", eval.unwrap());
    }
}