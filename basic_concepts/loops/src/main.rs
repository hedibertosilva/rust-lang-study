fn main() {

    for b in (0..=4).rev() {
        println!("the value is: {}", b)
    }
}


/*

    There are 3 types:

        * loop
        * while
        * for
    
    You can use the keywork break to stop the loop.

    * LOOP

        As the loop is expression declaration we can use loop to 
        return values.

        fn main() {
            let mut counter = 0;

            let result = loop {
                counter += 1;

                if counter == 10 {
                    break counter *2;
                    // you can add the value you want returned after the break expression 
                    // you use to stop the loop; that value will be returned out of the loop 
                    // so you can use it.
                }
            }
        }

    * WHILE 

        fn main() {
            let mut number = 3;

            while number != 0 {
                number -= 1;
                println!("{}!", number);
            }

            println!("LIFTOFF!!!")
        }

    * FOR

        fn main() {
            let a = [10, 20, 30, 40, 50];

            for b in a.iter() {
                println!("the value is: {}", b)
            }
        }

        fn main() {
            for b in 0..=4 {
                println!("the value is: {}", b);
            }
        }

        fn main() {
            for b in (1..4).rev() {
                println!("the value is: {}", b);
            }
        }

*/