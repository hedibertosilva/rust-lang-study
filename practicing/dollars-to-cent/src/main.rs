use std::io;

fn main() {

    let mut new_input = String::new();

    loop {
        println!("Dolar:");

        io::stdin()
            .read_line(&mut new_input)
            .expect("Failed Input");

        let new_input:f64 = match new_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("{}", new_input);
    }
}
