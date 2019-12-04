use std::io;

fn main() {

    loop
    {
        let mut input = String::new();

        println!("\nPlease input a temperature (e.g. 75F or -10C)");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input 😞");

        let input = input.trim();

        if input == "quit" 
        {
            break;
        }

        let (temperature, scale) = input.split_at(input.len() - 1);

        let temperature: f64 = match temperature.parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("Converted temperature: ");

        match scale
        {
            "C" => print!("{:.2}F", celsius_to_fahrenheit(temperature)),
            "F" => print!("{:.2}C", fahrenheit_to_celsius(temperature)),
            _ => continue,
        };
        println!();
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    const FIVE_NINTHS: f64 = 5.0/9.0;
    return (fahrenheit - 32.0) * FIVE_NINTHS;
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    return celsius * 1.8 + 32.0;
}