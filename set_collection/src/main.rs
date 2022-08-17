use std::collections::HashSet;

fn main() {
    let mut flights = HashSet::new();

    flights.insert(("DA113", "Boston", "06:20"));
    flights.insert(("DA98", "London", "09:43"));
    flights.insert(("DA428", "Salt Lake City", "12:05"));
    flights.insert(("DA41", "Berlin", "15:30"));
    flights.insert(("DA2815", "Nashville", "17:11"));

    for flight in flights.iter() {
        println!("{:?}", flight);
    }
}
