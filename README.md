# calc_lib

A simple library for passing in expressions in string from and getting back the result, using correct order of operations.\
This works best when you have an equation in string form, usually from user input, and need to solve it quickly and efficiently.\

# Examples:
Integer equations:
```rust
// solves a simple equation
use calc_lib::solve;

fn main() {
    // define the expression
    let expression = "(1 + 2) * 3";
    // solve the expression
    // the expression can also be defined in the parameters, and it can be anything that has the Into<String> trait
    let x = solve(expression);
    // handle errors that may occur
    if x.is_err() {
        // the custom errors implement display, so printing them out is extremely easy
        panic!("{}", x.unwrap_err());
    }
    // this will print "Result: 7"
    println!("Result: {}", x.unwrap());
}
```
Decimal Equations:
```rust
use calc_lib::solve_decimals;

fn main() {
    // define the expression
    let expression = "1.3 + 2.5 * 3.1";
    // solve the expression
    let x = solve_decimals("1.3 + 2.5 * 3.1");
    // handle errors that may occur
    if x.is_err() {
        panic!("{}", x.unwrap_err());
    }
    // this will print "Result: 9.05"
    println!("Result: {}", x.unwrap());
}
```
Solving with variables:
```rust
use calc_lib::{solve_with_definitions, Definitions};

fn main() {
    // define the expression
    let expression = "x + y";

    // define the variables
    // obviously this would be done differently depending on your use case,
    // this is just an example
    let mut definitions = Definitions::new();
    defs.insert("x".to_string(), 4);
    defs.insert("y".to_string(), 5);

    // solve the expression, and pass in the variable definitions
    let solved = solve_with_definitions_f64(expression, &definitions);
    // handle errors that may occur
    if solved.is_err() {
        panic!("{}", solved.unwrap_err());
    }
    // this will print "Result: 9"
    println!("Result: {}", solved.unwrap());
}
```
Solving with variables and decimals:
```rust
use calc_lib::{solve_with_definitions_f64, Definitions};

fn main() {
    // define the expression
    let expression = "x + y";

    // define the variables
    // obviously this would be done differently depending on your use case,
    // this is just an example
    let mut definitions = Definitions::new();
    defs.insert("x".to_string(), 4.5);
    defs.insert("y".to_string(), 5.5);

    // solve the expression, and pass in the variable definitions
    let solved = solve_with_definitions_f64(expression, &definitions);
    // handle errors that may occur
    if solved.is_err() {
        panic!("{}", solved.unwrap_err());
    }
    // this will print "Result: 10"
    println!("Result: {}", solved.unwrap());
}
```