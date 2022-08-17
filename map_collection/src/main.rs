use std::collections::HashMap;

fn main() {
    let mut flights = HashMap::new();

    flights.insert("DA113", ("Boston", "06:20"));
    flights.insert("DA98", ("London", "09:43"));
    flights.insert("DA428", ("Salt Lake City", "12:05"));
    flights.insert("DA41", ("Berlin", "15:30"));
    flights.insert("DA2815", ("Nashville", "17:11"));

    let flight_number = "DA531";

    // let option = flights.get(flight_number);
    // let (destination, time) = option.unwrap();

    // println!("{} {}", time, destination);

    if !flights.contains_key(flight_number) {
        flights.insert(flight_number, ("Puerto Rico", "12:00"));
    } else {
        println!("Flight {} is already entered", flight_number);
    }

    for flight in flights.iter() {
        println!("{:?}", flight);
    }
}
