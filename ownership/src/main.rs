fn main() {
    let mut original = String::from("original value");
    println!("\nOuter scope Original: \t\"{}\"\n", original);

    // let next = original;
    // next takes ownership of original and original is dropped from the memory
    {
        let next = &mut original;
        *next = String::from("Next Value");

        println!("Inner scope next: \t\"{}\"", next);
        println!("Inner scope original: \t\"{}\"", original);
    }
    println!("Outer scope Original: \t\"{}\"\n", original);

    // borrowing allows another variable to take temporary ownership of data without deallocating the original variable.
}
