The Rust Programming Language
=============================

This is a collection of projects based on the book The Rust Programming Language hosted on rust-lang.org.

My goals is to keep my progress registered.


## Notes

- Rust is an expression-based language

#### Structure

```
        std::io::Result
        ^     ^       ^
    library::method::submethod
    io::stdin()
    ^        ^
    method::associated function
```
#### Library Call

```
    `use std::io` can be optional that can be changed by:
    std::io::stdin()
    As well, we can use the word `as` to define the alias to the method.
```
#### Variables
```
    By default, all the variables are created as immutable 
    but we can change this using the work mut after let word.

    let name = 'Hediberto' // immutable
    let mut name = 'Hediberto' // mutable
```
#### References
```
    &name  // immutable reference
    &mut name // mutable reference    
```

#### Scalar Types

    Signed = -2^(n-1) ~ +2^(n-1)-1
    Unsigned = 0 ~ 2^(n)-1

    * Integers
        Length  Signed  Unsigned
        8-bit   i8      u8
        16-bit  i16     u16
        32-bit  i32     u32
        64-bit  i64     u64
        128-bit i128    u128
        arch    isize   usize
        
        Number literals     Example
        Decimal             98_222
        Hex                 0xff
        Octal               0o77
        Binary              0b1111_0000
        Byte (u8 only)      b'A'
        
    * Floating-Point
        There are two primitive types for floating-point numbers.
        f32 = 32 bits -> single-precision
        f64 = 64 bits -> double-precision
        fn main() {
            let a:f32 = 2.0;
            let b:f64 = 2.2;
            let c = 2.4; // default f64
        }
        
    * Numeric Operations
        fn main() {
            // addition
            let sum = 5 + 10;
            // subtraction
            let difference = 95.5 - 4.3;
            // multiplication
            let product = 4 * 30;
            // division
            let quotient = 56.7 / 32.2;
            // remainder
            let remainder = 43 % 5;
        }
        
    * Booleans = 8 bits = 1 byte
        fn main() {
            let t = true;
            let f: bool = false; // with explicit type annotation
        }
        
    * Characters = 32 bits = 4 bytes
        char literals are specified with single quotes, as opposed to string literals, which use double quotes.
        fn main() {
            let c = 'z';
            let z = 'ℤ';
            let heart_eyed_cat = '😻';
        }
#### Compound Types

    * Tuple
        It's fixed lenght. 
        fn main() {
            let tup: (i32, f64, u8) = (500, 6.4, 1);
        }
        It's possible to destructure a tup.
        fn main() {
            let example = (1, 2, 3);
            let (x, y, z) = example;
            println!("{}", x); // 1
            println!("{}", y); // 2
            println!("{}", z); // 3
        }
        It's possible to access the values directly.
        fn main() {
            let example = (1, 2, 3);
            
            println!("{}", example.0); // 1
            println!("{}", example.1); // 2
            println!("{}", example.2); // 3
        }
        
    * Array
        All values in array must have the same type. 
        Arrays in Rust are different  from arrays in some other languages because
        arrays in Rust have a fixed lenght, like tuples.
        fn main() {
            let a[u8; 5] = [1, 2, 3, 4, 5];
            let first = a[0];
            let second = a[1];
        }
        Obs! dont accept negative nubmers to access the array!
        ** Generating Arrays
        fn main() {
            let _a = [0; 5]; // [0, 0, 0, 0, 0]
        }
    
#### Functions

    The functions can be defined somewhere.

    * Functions can also be defined to have parameters. If they there are
    parameters all of them must to declare each parameter type.

    fn another_function(x: usize) {
        println!("The values of x is: {}", x);
    }

    Difference between Statement and Expression declaration.

    - Statement are instructions that perform some action and do not return a value. 

    - Expressions evaluate to a resulting value.

    fn main() {
        let x = 5;

        let y = {
            let x = 3;
            x+1
        };

        println!("The value of x is: {}", x);
        println!("The value of y is: {}", y);
    }

    -*-

    fn main() {
        let x = five();
        println!("x is equal {}", x);
    }

    fn five() -> u8 {
        5
    }

#### IF ELSE

    The condition needs must be a bool.

    Rust will not automatically try to convert non-Boolean types to a Boolean.


    - Structure:

        if <condiction> {
            <expression>
        } else if <condiction> {
            <expression>
        } else {
            <expression>
        }

        or

        let number = if condiction { 5 } else { 6 };

    Using too many else if expressions can clutter your code, 
    so if you have more than one, you might want to refactor your code. 
    Rust branching construct called match for these cases.

    All expressions must have the same type.