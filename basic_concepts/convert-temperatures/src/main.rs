fn main() {
    loop {
        println!("Type the temperature in Celsius:");
        
        let mut temp_celsius = String::new();

        std::io::stdin()
                 .read_line(&mut temp_celsius)
                 .expect("Please, digit a valid temperature!");

        let temp_celsius:f64 = match temp_celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        let temp_fahrenheit = (temp_celsius * 9.0/5.0) + 32.0;
        
        println!("{} Celsius is equal {} Fahrenheit.", temp_celsius, temp_fahrenheit);

    }
}