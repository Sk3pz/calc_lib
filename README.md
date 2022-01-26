# calc_lib

A crate for evaluating algebraic expressions from input using correct order of operations.\
This was designed originally for use in terminal based calculator apps.

### Features

* Basic algebraic operations
* Proper order of operations (functions are always evaluated first, then PEMDAS)
* Optional defined variables
* Integer operations and floating point operations (either/or)
* Functions such as log, sin, cos, tan, etc.
* Optional defined functions

### Planned Features

* equation validation (such as `2 + 2 = 4` which is valid, and `2 + 2 = 5` which is not)
* solving for a variable (such as `x + 2 = 4` will result in `x = 2`)

### Features that may be implemented in the future

* solving for multiple variables (such as `3x - y = 7`, `2x + y = 8` will result in `x = 3`, `y = 2`)

### Default functions
accessed with `Functions::default();`
* `log(base, value)`
* `sqrt(value)`
* `sin(value)`
* `cos(value)`
* `tan(value)`

### Custom Error system:

* Exposes the Error enum which allows the user to determine what type of error occurred, and have all the relevant information about it
* Allows for the user to handle errors in their own way if needed, but they can also just be printed out.

# Examples:
Integer equations:
```rust
// evaluates an algebraic equation
use calc_lib::evaluate;

fn main() {
    // the equation to evaluate
    let eval = evaluate("1 + 2 * 3");
    // print out errors if they occur, or handle them another way
    if eval.is_err() {
        panic!("{}", eval.err().unwrap());
    }
    assert_eq!(eval.unwrap() as i32, 7);
}
```
Decimal Equations:
```rust
use calc_lib::evaluate;

fn main() {
    // define the expression
    let expression = "1.3 + 2.5 * 3.1";
    // solve the expression
    let eval = evaluate(expression);
    // handle errors that may occur
    if eval.is_err() {
        panic!("{}", x.unwrap_err());
    }
    assert_eq!(eval.unwrap(), 9.05);
}
```
Solving with variables:
```rust
use calc_lib::{evaluate_with_defined, Definitions, Functions, Error};

fn main() {
    // define x as 16
    let mut defs = Definitions::new();
    defs.register("x", 16);
  
    // create the functions list
    // this defines an empty Functions struct with no functions.
    // for functions like log, sqrt, sin, cos, tan, etc., use `Functions::default()`
    let mut funcs = Functions::new();
    // this shows the definition of the log function,
    // exactly how it is implemented in `Functions::default();`
    funcs.register("log", |args| {
        // args is of type Vec<f64>
        // this takes 2 arguments: base, number
        if args.len() != 2 {
            return Err(Error::arg_count("log", 2, args.len()));
        }
        // return the value
        Ok(args[1].log(args[0]))
    });
    let eval = evaluate_with_defined("log(2, x)", Some(&defs), Some(&funcs));
    if eval.is_err() { 
      panic!("{}", eval.unwrap_err());
    }
    assert_eq!(eval.unwrap(), 4.0);
}
```
