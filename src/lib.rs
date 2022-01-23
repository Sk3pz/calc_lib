use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::input_reader::InputReader;
use crate::interpret::{interpret, interpret_with_definitions};

pub(crate) mod lex;
pub(crate) mod input_reader;
pub(crate) mod postfix;
pub(crate) mod interpret;
pub(crate) mod operator;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct InputPos {
    line: usize,
    ch: usize,
}

impl InputPos {
    pub(crate) fn new(line: usize, ch: usize) -> Self {
        Self {
            line, ch
        }
    }

    pub(crate) fn next(&mut self) {
        self.ch += 1
    }

    pub(crate) fn newline(&mut self) {
        self.line += 1;
        self.ch = 0;
    }
}

impl Default for InputPos {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

impl Display for InputPos {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.ch)
    }
}

/// An enum representing an error that occurred
/// This is used by the Error struct to represent errors
/// This allows for user handling of errors while still allowing them to just be
/// printed out if custom handling of errors is not needed.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ErrorType {
    /// An error in which the input attempts a division by zero, which is undefined
    DivByZero,
    /// An error in which the input attempts to raise a number to a negative power, which is undefined
    NegativeExponent,
    /// An error in which the input contains an invalid character that can not be parsed
    /// c is the invalid character
    InvalidCharacter { c: char },
    /// An error in which the input contains an invalid number (i.e. 2 decimal points)
    /// found is the number in string form (it can't be parsed)
    InvalidNumber { found: String },
    /// An error in which the parser expected something but got something else (invalid input)
    /// expected is the expected input
    /// found is what was found
    Expected { expected: String, found: String },
    /// when the parser reaches an unexpected End Of Input when it is still expecting more input
    UnexpectedEOI,
    /// When the interpreter finds an invalid operand, such as a variable identifier when none are defined
    /// op: the invalid operand
    InvalidOperand { op: String },
    /// When the interpreter finds an invalid operator, such as a '=' while solving
    /// op: the invalid operator
    InvalidOperator { op: String },
    /// When the interpreter has an invalid output, such as multiple values or a remaining operator
    /// reason: the error message
    InvalidExpression { reason: String },
    /// When interpreting with definitions finds an undefined variable
    /// name: the name of the undefined variable
    UndefinedVariable { name: String },
    /// When interpreting with definitions finds an undefined function
    /// name: the name of the undefined function
    UndefinedFunction { name: String },
    /// When a function is called with the wrong number of arguments
    /// name: the name of the function
    /// expected: the number of arguments expected
    /// got: the number of arguments received
    InvalidArgumentCount { name: String, expected: usize, got: usize },
    /// When something other than a number or variable is passed to a function
    /// name: the name of the function
    /// value: the value that was passed
    InvalidArgument { name: String, value: String },
    /// When the interpreter finds an operator after another operator that should not be there
    /// op: the operator that was found
    InvalidLeadingOperator { op: String },
    /// When the interpreter expects an operator (i.e. after a number) but gets something else
    MissingOperator,
    /// When there is an incomplete pair of parentheses (i.e. open with no close or vice versa)
    /// found: the parenthesis that was found (either '(' or ')')
    /// missing: the parenthesis that was expected (either ')' or '(')
    MismatchedParentheses { found: char, missing: char },
    /// Custom error messages
    /// contains a String of the error message
    /// this is not used by this program and is only used for custom error messages by the user
    ///
    Other(String),
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::DivByZero => write!(f, "Can't divide by zero"),
            ErrorType::NegativeExponent => write!(f, "Can't raise a value to a negative power"),
            ErrorType::InvalidCharacter { c } => write!(f, "Invalid character: {}", c),
            ErrorType::InvalidNumber { found } => write!(f, "Invalid number: {}", found),
            ErrorType::Expected { expected, found } => write!(f, "Expected '{}', found '{}'", expected, found),
            ErrorType::UnexpectedEOI => write!(f, "Unexpected end of input"),
            ErrorType::InvalidOperand { op } => write!(f, "Invalid operand: {}", op),
            ErrorType::InvalidOperator { op } => write!(f, "Invalid operator: {}", op),
            ErrorType::InvalidExpression { reason } => write!(f, "Invalid expression: {}", reason),
            ErrorType::UndefinedVariable { name } => write!(f, "Undefined variable: {}", name),
            ErrorType::UndefinedFunction { name } => write!(f, "Undefined function: {}", name),
            ErrorType::InvalidArgumentCount { name, expected, got } => write!(f, "Invalid argument count for function '{}': expected {}, got {}", name, expected, got),
            ErrorType::InvalidArgument { name, value } => write!(f, "Invalid argument for function '{}': {}", name, value),
            ErrorType::InvalidLeadingOperator { op } => write!(f, "Invalid leading operator: {}", op),
            ErrorType::MissingOperator => write!(f, "Missing operator"),
            ErrorType::MismatchedParentheses { found, missing } => write!(f, "Mismatched parentheses: found '{}', missing '{}'", found, missing),
            ErrorType::Other(s) => write!(f, "{}", s),
        }
    }
}

/// Stores errors from the parser if any occur.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    error: ErrorType,
    pos: Option<InputPos>,
}

impl Error {
    pub(crate) fn new(etype: ErrorType, pos: InputPos) -> Self {
        Self {
            error: etype,
            pos: Some(pos),
        }
    }

    pub(crate) fn new_gen(etype: ErrorType) -> Self {
        Self {
            error: etype,
            pos: None,
        }
    }

    /// a way for users to create new errors
    pub fn create(etype: ErrorType) -> Self {
        Self::new_gen(etype)
    }

    /// Returns the error message.
    pub fn get_error(&self) -> &ErrorType {
        &self.error
    }

    /// returns the location of the error as (line, character)
    /// this struct uses a private `InputPos` struct to store the position,
    /// but that is not exposed to the user.
    pub fn get_loc(&self) -> (usize, usize) {
        (self.pos.as_ref().unwrap().line, self.pos.as_ref().unwrap().ch)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.pos.is_some() {
            write!(f, "Error: {} at {}", self.error, self.pos.as_ref().unwrap())
        } else {
            write!(f, "Error: {}", self.error)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Number {
    value: f64,
}

impl Number {
    /// Create a new number from a value
    pub fn new<N: Into<f64>>(n: N) -> Self {
        Self {
            value: n.into(),
        }
    }

    /// invert the sign of the number
    pub fn negate(&mut self) {
        self.value = -self.value;
    }

    /// get the value of the number as an f64
    pub fn as_f64(&self) -> f64 {
        self.value
    }

    /// get the value of the number as an i128
    pub fn as_i128(&self) -> i128 {
        self.value.round() as i128
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.value.fract() == 0.0 {
            self.as_i128().to_string()
        } else {
            self.value.to_string()
        })
    }
}

/// A list of definitions to pass into the crate to be used in the interpreter.
pub struct Definitions {
    pub(crate) map: HashMap<String, Number>,
}

impl Definitions {
    /// Create a new definition map
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// register a new definition to the map
    pub fn register<S: Into<String>>(&mut self, name: S, value: Number) {
        self.map.insert(name.into(), value);
    }

    pub fn exists<S: Into<String>>(&self, ident: S) -> bool {
        self.map.contains_key(ident.into().as_str())
    }

    /// Get a definition from the map
    pub(crate) fn get<S: Into<String>>(&self, ident: S) -> Option<&Number> {
        self.map.get(ident.into().as_str())
    }
}

/// A list of definitions of functions to pass into the interpreter to solve for the variables.
pub struct Functions<'a> {
    pub(crate) functions: HashMap<String, Box<dyn Fn(Vec<Number>) -> Result<Number, Error> + 'a>>,
}

impl<'a> Functions<'a> {
    /// Create a new list of functions
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    /// register a function
    pub fn register<S: Into<String>, F: Fn(Vec<Number>) -> Result<Number, Error> + 'a + Copy>(&mut self, name: S, f: F) {
        self.functions.insert(name.into(), Box::new(f));
    }

    /// check if a function exists
    pub fn exists<S: Into<String>>(&self, ident: S) -> bool {
        self.functions.contains_key(ident.into().as_str())
    }

    pub(crate) fn get<S: Into<String>>(&self, ident: S) -> Option<&Box<dyn Fn(Vec<Number>) -> Result<Number, Error> + 'a>> {
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
                return Err(Error::create(ErrorType::InvalidArgumentCount { name: "log".to_string(), expected: 2, got: args.len() }));
            }
            Ok(Number::new(args[1].as_f64().log(args[0].as_f64())))
        });

        funcs.register("sqrt", |args| {
            if args.len() != 1 {
                return Err(Error::create(ErrorType::InvalidArgumentCount { name: "sqrt".to_string(), expected: 1, got: args.len() }));
            }
            Ok(Number::new(args[0].as_f64().sqrt()))
        });

        funcs.register("sin", |args| {
            if args.len() != 1 {
                return Err(Error::create(ErrorType::InvalidArgumentCount { name: "sin".to_string(), expected: 1, got: args.len() }));
            }
            Ok(Number::new(args[0].as_f64().sin()))
        });

        funcs.register("cos", |args| {
            if args.len() != 1 {
                return Err(Error::create(ErrorType::InvalidArgumentCount { name: "cos".to_string(), expected: 1, got: args.len() }));
            }
            Ok(Number::new(args[0].as_f64().cos()))
        });

        funcs.register("tan", |args| {
            if args.len() != 1 {
                return Err(Error::create(ErrorType::InvalidArgumentCount { name: "tan".to_string(), expected: 1, got: args.len() }));
            }
            Ok(Number::new(args[0].as_f64().tan()))
        });

        funcs
    }
}

/// Solves an equation in infix notation using the shunting yard algorithm.
/// this function will not accept decimals numbers, only integers.
/// # Usage Example:
/// ```
/// use calc_lib::solve;
///
/// let x = solve("(1 + 2) * 3");
/// if x.is_err() {
///     panic!("{}", x.unwrap_err());
/// }
/// assert_eq!(x.unwrap().as_i128(), 9);
/// ```
///
pub fn solve<S: Into<String>>(input: S) -> Result<Number, Error> {
    let mut input = InputReader::new(input.into());
    let mut tokens = lex::lex(&mut input, false)?;
    let mut shunted = postfix::shunting_yard(&mut tokens)?;
    interpret(&mut shunted)
}

/// Solves an equation in infix notation using the shunting yard algorithm.
/// This will not accept decimal numbers, only integers.
/// this function takes a HashMap of definitions (type Definitions<i128>)
/// and will replace identifiers found in the equation with their respective values.
///
/// # Usage Example:
/// ```
/// use calc_lib::{Definitions, Number, solve_defs};
///
/// let mut defs = Definitions::new();
/// defs.register("x", Number::new(3));
///
/// let solved = solve_defs("(x + 3) / 3", Some(&defs), None);
/// assert_eq!(solved.unwrap().as_i128(), 2);
/// ```
///
/// # Usage with functions:
/// ```
/// use calc_lib::{Definitions, Functions, Number, Error, solve_defs, ErrorType};
///
/// let mut defs = Definitions::new();
/// defs.register("x", Number::new(3));
///
/// let mut funcs = Functions::new();
/// // this shows the definition of the log function, which is already implemented in `Functions::default();`
/// funcs.register("log", |args| {
///     if args.len() != 2 {
///         return Err(Error::create(ErrorType::InvalidArgumentCount { name: "log".to_string(), expected: 2, got: args.len() }));
///     }
///     Ok(Number::new(args[1].as_f64().log(args[0].as_f64())))
///  });
///
/// let solved = solve_defs("log(2, 16)", Some(&defs), Some(&funcs));
/// assert_eq!(solved.unwrap().as_i128(), 4);
/// ```
///
pub fn solve_defs<S: Into<String>>(input: S, definitions: Option<&Definitions>, functions: Option<&Functions>) -> Result<Number, Error> {
    let mut input = InputReader::new(input.into());
    let mut tokens = lex::lex(&mut input, true)?;
    let mut shunted = postfix::shunting_yard(&mut tokens)?;
    interpret_with_definitions(&mut shunted, definitions, functions)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_shunt() {
        let solved = solve("1 + 2 * 3");
        if solved.is_err() {
            panic!("{}", solved.err().unwrap());
        }
        assert_eq!(solved.unwrap().as_i128(), 7);

        let solved2 = solve("");
        if solved2.is_err() {
            panic!("{}", solved2.err().unwrap());
        }
        assert_eq!(solved2.unwrap().as_i128(), 0);

        let x = solve("1.3 + 2.5 * 3.1");
        if x.is_err() {
            panic!("{}", x.unwrap_err());
        }
        assert_eq!(x.unwrap().as_f64(), 9.05);

        let mut defs = Definitions::new();
        defs.register("x", Number::new(16));

        let solved3 = solve_defs("(x + 4) / 5.0", Some(&defs), None);
        assert_eq!(solved3.unwrap().as_f64(), 4.0);

        let funcs = Functions::default();
        let solved4 = solve_defs("log(2, x)", Some(&defs), Some(&funcs));
        if solved4.is_err() {
            panic!("{}", solved4.unwrap_err());
        }
        assert_eq!(solved4.unwrap().as_f64(), 4.0);

        let solved5 = solve_defs("log(log(2,4), x)", Some(&defs), Some(&funcs));
        if solved5.is_err() {
            panic!("{}", solved5.unwrap_err());
        }
        assert_eq!(solved5.unwrap().as_f64(), 4.0);
    }
}