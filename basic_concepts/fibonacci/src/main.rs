fn main() {

    let mut last_but_one:usize = 0;
    let mut last:usize = 1;

    let mut counter:u8 = 0;

    println!("{}", last_but_one);
    println!("{}", last);

    while counter < 15 {
        let result:usize = sum(last, last_but_one);

        println!("{}", result);

        last_but_one = last;
        last = result;
        
        counter += 1;
    }

}

fn sum(x: usize, y: usize) -> usize {
    x + y
}