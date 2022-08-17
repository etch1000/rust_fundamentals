fn main() {
    let greater = return_greater(10, 5);
    println!("{greater}");

    let mut original = String::from("original value");

    println!("\nOuter scope original: \t\"{}\"", original);

    {
        print_original(&original);
        change_original(&mut original);
        println!("Inner scope original: \t\"{}\"", original);
    }
}

fn return_greater(first: u8, second: u8) -> u8 {
    if first > second {
        first
    } else {
        second
    }
}

// Pass by Value or Pass by Reference

fn print_original(original: &String) {
    println!("fn print_original: \t\"{}\"", original);
}

fn change_original(original: &mut String) {
    let next = original;
    *next = String::from("next value");
    println!("fn change_original: \t\"{}\"", next);
}
