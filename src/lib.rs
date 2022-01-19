use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use better_term::{Color, Style};
use crate::input_reader::InputReader;
use crate::interpret::{interpret, interpret_f64, interpret_with_definitions, interpret_with_definitions_f64};

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

/// A list of definitions to pass into the crate to be used in the interpreter.
pub type Definitions<T> = HashMap<String, T>;

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
/// assert_eq!(x.unwrap(), 9);
/// ```
///
pub fn solve<S: Into<String>>(input: S) -> Result<i128, Error> {
    let mut input = InputReader::new(input.into());
    let mut tokens = lex::lex(&mut input, false)?;
    let mut shunted = postfix::shunting_yard(&mut tokens)?;
    interpret(&mut shunted)
}

/// Solves an equation in infix notation using the shunting yard algorithm.
/// this function will accept decimals and will convert integers to decimals.
/// # Usage Example:
/// ```
/// use calc_lib::solve_decimals;
///
/// let x = solve_decimals("1.3 + 2.5 * 3.1");
/// if x.is_err() {
///     panic!("{}", x.unwrap_err());
/// }
/// assert_eq!(x.unwrap(), 9.05);
/// ```
///
pub fn solve_decimals<S: Into<String>>(input: S) -> Result<f64, Error> {
    let mut input = InputReader::new(input.into());
    let mut tokens = lex::lex(&mut input, false)?;
    let mut shunted = postfix::shunting_yard(&mut tokens)?;
    interpret_f64(&mut shunted)
}

/// Solves an equation in infix notation using the shunting yard algorithm.
/// This will not accept decimal numbers, only integers.
/// this function takes a HashMap of definitions (type Definitions<i128>)
/// and will replace identifiers found in the equation with their respective values.
///
/// # Usage Example:
/// ```
/// use calc_lib::{Definitions, solve_with_definitions};
///
/// let mut defs = Definitions::new();
/// defs.insert("x".to_string(), 3);
///
/// let solved = solve_with_definitions("(x + 3) / 3", &defs);
/// assert_eq!(solved.unwrap(), 2);
/// ```
///
pub fn solve_with_definitions<S: Into<String>>(input: S, definitions: &Definitions<i128>) -> Result<i128, Error> {
    let mut input = InputReader::new(input.into());
    let mut tokens = lex::lex(&mut input, true)?;
    let mut shunted = postfix::shunting_yard(&mut tokens)?;
    interpret_with_definitions(&mut shunted, definitions)
}

/// Solves an equation in infix notation using the shunting yard algorithm.
/// This accepts decimal numbers, and will convert passed in integer numbers to decimal.
/// this function takes a HashMap of definitions (type Definitions<f64>)
/// and will replace identifiers found in the equation with their respective values.
///
/// # Usage Example:
/// ```
/// use calc_lib::{Definitions, solve_with_definitions_f64};
///
/// let mut defs = Definitions::new();
/// defs.insert("x".to_string(), 4.5);
///
/// let solved = solve_with_definitions_f64("(x + 4.5) / 3", &defs);
/// assert_eq!(solved.unwrap(), 3.0);
/// ```
///
pub fn solve_with_definitions_f64<S: Into<String>>(input: S, definitions: &Definitions<f64>) -> Result<f64, Error> {
    let mut input = InputReader::new(input.into());
    let mut tokens = lex::lex(&mut input, true)?;
    let mut shunted = postfix::shunting_yard(&mut tokens)?;
    interpret_with_definitions_f64(&mut shunted, definitions)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_shunt() {
        let solved = solve_decimals("1 + 2 * 3");
        if solved.is_err() {
            panic!("{}", solved.err().unwrap());
        }
        assert_eq!(solved.unwrap(), 7.0);
        let x = solve_decimals("1.3 + 2.5 * 3.1");
        if x.is_err() {
            panic!("{}", x.unwrap_err());
        }
        assert_eq!(x.unwrap(), 9.05);

        let mut defs = Definitions::new();
        defs.insert("x".to_string(), 4.5);

        let solved3 = solve_with_definitions_f64("(x + 4.5) / 3.0", &defs);
        assert_eq!(solved3.unwrap(), 3.0);
    }
}