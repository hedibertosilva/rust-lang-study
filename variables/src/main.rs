fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "Hi";
    println!("The value of x is: {}", x);
}

/*
    In the mutable variable we dont can
    change the type of value.

    * Wrong way:
        let mut x = 5;
        x = "Hi";

    But if we redeclare the variable with let 
    we can change the type of value.

    * Accept way:
        let x = 5;
        let x = "Hi";

    The difference between `let` and `const`:

    * Const:
        - It's assign on build time. 
        - It's always immutable.

        const MAX_POINTS: u32 = 100_000;

        - const not cam be defined from result of a funcion call.

*/