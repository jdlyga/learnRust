fn main() {

    let days: [&str; 12] = ["first", 
                            "second", 
                            "third", 
                            "fourth", 
                            "fifth", 
                            "sixth", 
                            "seventh", 
                            "eighth", 
                            "ninth", 
                            "tenth", 
                            "eleventh", 
                            "twelfth"];

    let twelve_days_of_christmas: [&str; 12] = ["a partridge in a pear tree", 
                                                "Two turtle doves, and",
                                                "Three french hens",
                                                "Four calling birds",
                                                "Five golden rings",
                                                "Six geese a-laying",
                                                "Seven swans a-swimming",
                                                "Eight maids a-milking",
                                                "Nine ladies dancing",
                                                "Ten lords a-leaping",
                                                "Eleven pipers piping",
                                                "Twelve drummers drumming"];

    for verse in 0..12 {
        println!("On the {} day of christmas my true love gave to me", days[verse]);

        for index in (0..=verse).rev() {
            println!("{}", twelve_days_of_christmas[index]);
        }
        println!();
    }
}
