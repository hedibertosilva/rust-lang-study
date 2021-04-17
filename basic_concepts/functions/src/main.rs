fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);   
}

fn plus_one(x: u8) -> u8 {
    x + 1
}

/*

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
    
    fn main() {
        let x = five();
        println!("x is equal {}", x);
    }

    fn five() -> u8 {
        5
    }
*/