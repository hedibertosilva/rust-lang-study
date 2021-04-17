fn main() {
    let condiction = false;
    let number = if condiction { 5 } else { 6 };
    
    println!("The value of number is: {}", number);

}

/*

    * IF ELSE

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


*/