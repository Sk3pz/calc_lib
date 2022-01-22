# calc_lib

A simple library for passing in expressions in string from and getting back the result, using correct order of operations.\
This works best when you have an equation in string form, usually from user input, and need to solve it quickly and efficiently.

### Features

* Basic algebraic operations
* Proper order of operations
* Optional defined variables (definitions not handled by this library)
* Integer operations and floating point operations (either/or)
* Functions such as log, sin, cos, tan, etc.
* Optional defined functions (planned to be both infix and normal)

### Planned Features

* equation validation (such as `2 + 2 = 4` which is valid, and `2 + 2 = 5` which is not)
* variable solving (such as `x + 2 = 4` will result in `x = 2`)

### Features that may be implemented in the future

* Solving for multiple variables (such as `x + y = 4` `2x + 2y = 8` will result in `x = 2` `y = 2`)

### Planned changes

* Error system rework to allow for errors that the user can handle (represent errors as an enum instead of a String)

### Default functions
accessed with `Functions::default();`
* `log(base, value)`
* `sqrt(value)`
* `sin(value)`
* `cos(value)`
* `tan(value)`
* `atan(value)`
* `atan2(value, other)`

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
    defs.insert("x".to_string(), Number::new(16));
  
    // create the functions list
    // Functions::default(); adds functions like log, sin, cos, tan, etc.
    let funcs = Functions::default();
    let solved4 = solve_defs("log(2, x)", Some(&defs), Some(&funcs));
    if solved4.is_err() { 
      panic!("{}", solved4.unwrap_err());
    }
    assert_eq!(solved4.unwrap().as_f64(), 4.0);
}
```
