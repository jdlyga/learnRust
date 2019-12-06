use std::io;

//////////////////////////////////////////////////////////////////////////////////////////

fn main() {
    const MAX_PIZZAS: u32 = 187; //anything greater causes overflow in u128
    let mut input = String::new();

    println!("Welcome to Fibonacci's Pizza restaurant. How many pizzas would you like?");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let num_terms: u32 = input.trim().parse().expect("Not a numeric input");

    if num_terms <= 0 {
        println!("You gotta order at least one!");
        return;
    }

    if num_terms > MAX_PIZZAS {
        println!("Too many pizzas!");
        return;
    }

    fibonacci(num_terms);
}

//////////////////////////////////////////////////////////////////////////////////////////

/** prints out num_terms of the fibonacci sequence */
fn fibonacci(num_terms: u32) {
    let mut term_1: u128 = 0;
    let mut term_2: u128 = 1;
    let mut next_term: u128;
    let mut output: u128;

    for number in 1..=num_terms {
        if number == 1 {
            output = term_1;
        } else if number == 2 {
            output = term_2;
        } else {
            next_term = term_1 + term_2;
            term_1 = term_2;
            term_2 = next_term;

            output = next_term;
        }

        println!(
            "Order {:03}: Here is a pizza with {} {}",
            number,
            output,
            slice_or_slices(output)
        );
    }

    println!("Enjoy!");
}

//////////////////////////////////////////////////////////////////////////////////////////

/** returns a new string that's either singular or plural for grammatical correctness */
fn slice_or_slices(number: u128) -> String {
    let output = match number {
        1 => "slice".to_string(),
        _ => "slices".to_string(),
    };

    return output;
}
