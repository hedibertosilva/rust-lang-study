fn main() {
    let guess:u8 = "42".parse().expect("Parse failed!");
    println!("{}", guess);
    let _guess2 = 2.0;
}

/*

    Scalar Types

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


*/