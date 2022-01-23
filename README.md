# calc_lib

A simple library for passing in expressions in string from and getting back the result, using correct order of operations.\
This works best when you have an equation in string form, usually from user input, and need to solve it quickly and efficiently.

### Features

* Basic algebraic operations
* Proper order of operations (functions are always evaluated first, then PEMDAS)
* Optional defined variables
* Integer operations and floating point operations (either/or)
* Functions such as log, sin, cos, tan, etc.
* Optional defined functions

### Planned Features

* equation validation (such as `2 + 2 = 4` which is valid, and `2 + 2 = 5` which is not)
* variable solving (such as `x + 2 = 4` will result in `x = 2`)

### Features that may be implemented in the future

* Solving for multiple variables (such as `x + y = 4` `2x + 2y = 8` will result in `x = 2` `y = 2`)

### Default functions
accessed with `Functions::default();`
* `log(base, value)`
* `sqrt(value)`
* `sin(value)`
* `cos(value)`
* `tan(value)`

### Custom Error system:

* Exposes the ErrorType enum which allows the user to determine what type of error occurred, and have all the relevant information about it
* Allows for the user to handle errors in their own way if needed, but they can also just be printed out.

# Examples:
Integer equations:
```rust
// solves a simple equation
use calc_lib::solve;

fn main() {
    // the equation to solve
    let solved = solve("1 + 2 * 3");
    // print out errors if they occur, or handle them another way
    if solved.is_err() {
        panic!("{}", solved.err().unwrap());
    }
    assert_eq!(solved.unwrap().as_i128(), 7);
}
```
Decimal Equations:
```rust
use calc_lib::solve;

fn main() {
    // define the expression
    let expression = "1.3 + 2.5 * 3.1";
    // solve the expression
    let solved = solve(expression);
    // handle errors that may occur
    if solved.is_err() {
        panic!("{}", x.unwrap_err());
    }
    assert_eq!(solved.unwrap().as_f64(), 9.05);
}
```
Solving with variables:
```rust
use calc_lib::{solve_defs, Definitions, Number, Functions};

fn main() {
    // define x as 16
    let mut defs = Definitions::new();
    defs.register("x", Number::new(16));
  
    // create the functions list
    let mut funcs = Functions::new();
    // this shows the definition of the log function,
    // which is already implemented in `Functions::default();`
    funcs.register("log", |args| {
        // this function takes two arguments, base and the number
        if args.len() != 2 {
            // if the number of arguments is not 2, return an error
            return Err(Error::arg_count("log", 2, args.len()));
        }
        Ok(Number::new(args[1].as_f64().log(args[0].as_f64())))
     });
    let solved4 = solve_defs("log(2, x)", Some(&defs), Some(&funcs));
    if solved4.is_err() { 
      panic!("{}", solved4.unwrap_err());
    }
    assert_eq!(solved4.unwrap().as_f64(), 4.0);
}
```
