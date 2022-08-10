fn main() {
    let float_thirty_two: f32 = 18.26;
    let unsigned_eight: u8 = 9;
    let cast_unsigned_eight = unsigned_eight as f32;

    let result = float_thirty_two / cast_unsigned_eight;
    println!("{result}");

    // casting u8 as char
    let number: u8 = 69; // only u8 can be cast as char.
                         // no other type can be cast as char.
    let letter: char = number as char;

    println!("{letter}");
}
