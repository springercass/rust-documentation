// VARIABLES AND MUTABILITY
// variables must have mut in front of them in order to assign multiple values

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

// variables vs constants
// constants are values that are bound to a name and are not allowed to change
// they’re always immutable
// const MAX_POINTS: u32 = 100_000;

// SHADOWING
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

// DATA TYPES*******

// let guess: u32 = "42".parse().expect("Not a number!");
// if the data type annotator (u32) wasn't used, rust wouldn't know what to do with the guess

// SCALAR TYPES
// A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.

// --integers
// An integer is a number without a fractional component.
// 8-bit	i8	u8
// 16-bit i16 u16
// 32-bit i32 u32
// 64-bit i64 u64
// 128-bit i128 u128
// arch isize usize
// Signed and unsigned refer to whether it’s possible for the number to be negative or positive
// i = signed u = unsigned

// --floating point types
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
// The f32 type is a single-precision float, and f64 has double precision.

// tuple type compounds
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
// destructuring tuple
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
// This following program creates a tuple, x, and then makes new variables for each element by using their index. As with most programming languages, the first index in a tuple is 0.
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

// array compounds
fn main() {
    let a = [1, 2, 3, 4, 5];
}

let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

let a: [i32; 5] = [1, 2, 3, 4, 5];

let a = [3; 5];
// The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.


//FUNCTIONS
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

//function parameters
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
//In function signatures, you must declare the type of each parameter
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}


//function bodies containing statements and expressions****
// Creating a variable and assigning a value to it with the let keyword is a statement. In Listing 3-1, let y = 6; is a statement.
fn main() {
    let y = 6;
}

//functions with return values
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
//no semi colon in the return function


//CONTROL FLOW****
//similar to js
//--conditions
//----if else
//--multiple conditions
//----if else else if

//using if in a let
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

//REPETITION****
//loop, while, for

//loop
fn main() {
    loop {
        println!("again!");
    }
}

//returning values form loops
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

//conditional loops with WHILE
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

//looping with a collection with FOR
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}