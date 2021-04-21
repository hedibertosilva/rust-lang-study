use  std::io;

fn main() {

    loop {
                
        let input = read_customer_input();

        let dolar = match transform_to_dolar(&input) {
            Ok(num) => num,
            Err(_) => continue
        };

        let cents = dolar_to_cents(&dolar);
               
        let (quarters, dimes, nickels, pennies) = count_coins(&cents);

        println!("Total Cents: {}", cents);

        println!("Total Quarters: {}", quarters);
        println!("Total Dimes: {}", dimes);
        println!("Total Nickels: {}", nickels);
        println!("Total Pennies: {}", pennies);
    }
}

fn read_customer_input() -> String {
    println!("Please, enter a dolar value: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Please, enter a valid dolar value!");
    input
}

fn transform_to_dolar(input: &String) -> Result<f32, std::num::ParseFloatError> {
    match input.trim().parse() {
        Ok(num) => Ok(num),
        Err(e) => Err(e)
    }
}

fn dolar_to_cents(dolar: &f32) -> u32 {
    (dolar * 100.0) as u32
}

fn count_coins(cents: &u32) -> (u32, u32, u32, u32) {
    let quarters = count_quarters(cents);
    let dimes = count_dimes(cents, &quarters);
    let nickels = count_nickels(cents, &quarters, &dimes);
    let pennies = count_pennies(cents, &quarters, &dimes, &nickels);
    (quarters, dimes, nickels, pennies)
}

fn count_quarters(cents: &u32) -> u32 {
    cents / 25
}

fn count_dimes(cents: &u32, quarters: &u32) -> u32 {
    let remainder = cents % (25*quarters);
    remainder / 10
}

fn count_nickels(cents: &u32, quarters: &u32, dimes: &u32) -> u32 {
    let remainder = cents % (25*quarters + 10*dimes);
    remainder / 5
}

fn count_pennies(cents: &u32, quarters: &u32, dimes: &u32, nickels: &u32) -> u32 {
    let remainder = cents % (25*quarters + 10*dimes + 5*nickels);
    remainder / 1
}
