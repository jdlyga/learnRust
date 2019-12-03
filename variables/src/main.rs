const MAX_POINTS: u32 = 100_000;

fn main() {
    // variables are mutable by default
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("Max Points: {}", MAX_POINTS);

    //shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let mut emoji = '😻';
    println!("Your emoji: {}", emoji);
    emoji = '😁';
    println!("Your emoji: {}", emoji);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuppy tup: {} {} {}", tup.0, tup.1, tup.2);

    let tup = (560, "kong", 0x19);
    println!("Tuppy tup: {} {} {}", tup.0, tup.1, tup.2);    

    let joearray = [0, 1, 2, 3, 4, 6, 100];
    println!("array: {:?}", joearray);

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];    

    println!("Months: {:?}", months);

    let a = [0; 5];
    println!("{:?}", a);
}
