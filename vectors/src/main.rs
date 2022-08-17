fn main() {
    let mut flights: Vec<&str> = Vec::new();

    flights.push("DA113\tto Boston departs at 06:20");
    flights.push("DA98\tto London departs at 09:43");
    flights.push("DA428\tto Salt Lake City departs at 12:05");
    flights.push("DA41\tto Berlin departs at 15:30");
    flights.push("DA2815\tto Nashville departs at 17:11");

    let fourth = flights.get(3);

    match fourth {
        Some(flight_value) => {
            println!("Fourth Flight is : {flight_value}");
        }
        _ => {
            println!("No Flight");
        }
    }

    flights.insert(2, "DA94\tto Richmond departs at 11:12");
    flights.remove(1);

    for flight in flights.iter() {
        println!("{flight}");
    }
}
