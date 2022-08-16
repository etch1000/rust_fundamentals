fn main() {
    let phrase = String::from("Duck Airlines");

    // will show A
    let letter = phrase.chars().nth(15);
    // will show No Value
    // let letter = phrase.chars().nth(15);

    // println!("letter variable {:?}", letter);

    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No Value"),
    };

    println!("{value}");
}
