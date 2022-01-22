use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use better_term::{Color, Style};
use crate::input_reader::InputReader;
use crate::interpret::{interpret, interpret_with_definitions};

pub(crate) mod lex;
pub(crate) mod input_reader;
pub(crate) mod postfix;
pub(crate) mod interpret;
pub(crate) mod operator;

/// rounds a f64 to a specific decimal place
/// # Arguments
/// * value - the value to round
/// * places - the number of decimal places to round to
/// # Returns the rounded value
///
/// # Example
/// ```
/// use calc_lib::round;
///
/// assert_eq!(round(1.2345, 2), 1.23);
/// ```
pub fn round(value: f64, place: usize) -> f64 {
    let round_by = 10.0f64.powi(place as i32) as f64;
    (value * round_by).round() / round_by
}

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

/// Stores errors from the parser if any occur.
/// error: The message of the error. Accessed with `.get_error()`
/// pos: The position of the error. Accessed with `.get_pos()`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Error {
    error: String,
    pos: Option<InputPos>,
}

impl Error {
    pub(crate) fn new<S: Into<String>>(msg: S, pos: InputPos) -> Self {
        Self {
            error: msg.into(),
            pos: Some(pos),
        }
    }

    pub(crate) fn new_gen<S: Into<String>>(msg: S) -> Self {
        Self {
            error: msg.into(),
            pos: None,
        }
    }

    pub fn create<S: Into<String>>(msg: S) -> Self {
        Self::new_gen(msg.into())
    }

    /// Returns the error message.
    pub fn get_error(&self) -> &str {
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
        let strong_red = Style::default().fg(Color::Red).bold();
        let strong_white = Style::default().fg(Color::BrightWhite).bold();
        let reset = Style::reset();
        if self.pos.is_some() {
            write!(f, "{}error: {}{}{}\n  {}-> {}{}",
                   strong_red, strong_white, self.error, reset,
                   Color::BrightBlue, Color::BrightBlack, self.pos.as_ref().unwrap()
            )
        } else {
            write!(f, "{}error: {}{}{}",
                   strong_red, strong_white, self.error, reset)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Number {
    value: f64,
}

impl Number {
    pub fn new<N: Into<f64>>(n: N) -> Self {
        Self {
            value: n.into(),
        }
    }

    pub fn negate(&mut self) {
        self.value = -self.value;
    }

    pub fn as_f64(&self) -> f64 {
        self.value
    }

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
pub type Definitions = HashMap<String, Number>;

/// A list of definitions of functions to pass into the interpreter to solve for the variables.
pub struct Functions<'a> {
    pub(crate) functions: HashMap<String, Box<dyn Fn(Vec<Number>) -> Result<Number, Error> + 'a>>,
}

impl<'a> Functions<'a> {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    pub fn register<S: Into<String>, F: Fn(Vec<Number>) -> Result<Number, Error> + 'a + Copy>(&mut self, name: S, f: F) {
        self.functions.insert(name.into(), Box::new(f));
    }

    pub fn get<S: Into<String>>(&self, ident: S) -> Option<&Box<dyn Fn(Vec<Number>) -> Result<Number, Error> + 'a>> {
        let ident = ident.into();
        if !self.functions.contains_key(&ident) {
            return None;
        }
        self.functions.get(&ident)
    }
}

impl Default for Functions<'_> {
    fn default() -> Self {
        let mut funcs = Functions::new();
        funcs.register("log", |args| {
            if args.len() != 2 {
                return Err(Error::create(format!("log takes exactly 2 arguments, {} given", args.len())));
            }
            Ok(Number::new(args[1].as_f64().log(args[0].as_f64())))
        });

        funcs.register("sqrt", |args| {
            if args.len() != 1 {
                return Err(Error::create(format!("sqrt takes exactly 1 argument, {} given", args.len())));
            }
            Ok(Number::new(args[0].as_f64().sqrt()))
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
/// defs.insert("x".to_string(), Number::new(3));
///
/// let solved = solve_defs("(x + 3) / 3", Some(&defs), None);
/// assert_eq!(solved.unwrap().as_i128(), 2);
/// ```
///
/// # Usage with functions:
/// ```
/// use calc_lib::{Definitions, Functions, Number, Error, solve_defs};
///
/// let mut defs = Definitions::new();
/// defs.insert("x".to_string(), Number::new(3));
///
/// let mut funcs = Functions::new();
/// funcs.register("log", |args| {
///     if args.len() != 2 {
///         return Err(Error::create(format!("log takes exactly 2 arguments, {} given", args.len())));
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
        let x = solve("1.3 + 2.5 * 3.1");
        if x.is_err() {
            panic!("{}", x.unwrap_err());
        }
        assert_eq!(x.unwrap().as_f64(), 9.05);

        let mut defs = Definitions::new();
        defs.insert("x".to_string(), Number::new(16));


        let solved3 = solve_defs("(x + 4) / 5.0", Some(&defs), None);
        assert_eq!(solved3.unwrap().as_f64(), 4.0);

        let funcs = Functions::default();
        let solved4 = solve_defs("log(2, x)", Some(&defs), Some(&funcs));
        if solved4.is_err() {
            panic!("{}", solved4.unwrap_err());
        }
        assert_eq!(solved4.unwrap().as_f64(), 4.0);
    }
}