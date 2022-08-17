fn main() {
    for index in 1..11 {
        // for index in 1..=10 can be done too
        println!("{index}");
    }

    let duck_aircrafts = [
        "Boeing 737",
        "Boeing 767",
        "Boeing 787",
        "Airbus 319",
        "Airbus 320",
    ];

    for aircraft in duck_aircrafts.iter() {
        // we have to explicitly invoke the iter method of the collection.
        // iterator is going to keep track of where it is in the list of items.
        println!("{aircraft}");
    }
}
