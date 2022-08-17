fn main() {
    let name = "Duck Airlines";

    let write_message = |slogan: String| -> String {
        String::from(format!("{} : {}", name, slogan)) // function that doesn't have a name;
    };

    let phrase = write_message(String::from("We hit the gound every time."));

    println!("{phrase}");
}
