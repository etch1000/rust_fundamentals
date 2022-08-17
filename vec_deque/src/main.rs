// vecDeque stands for Vector Double Ended Queue
// Can add and remove from the front and the back
// We cannot sort the elements of vecDeque

use std::collections::VecDeque;

fn main() {
    let mut flights: VecDeque<&str> = VecDeque::new();

    flights.push_front("DA918\tto Orlando departs at 11:12");
    flights.push_back("DA428\tto Salt Lake City departs at 12:05");
    flights.push_front("DA98\tto London departs at 09:43");
    flights.push_front("DA113\tto Boston departs at 06:20");
    flights.push_back("DA41\tto Berlin departs at 15:30");
    flights.push_back("DA2815\tto Nashville departs at 17:11");

    let length = flights.len();
    println!("There are {length} flights:");

    for flight in flights.iter() {
        println!("{flight}");
    }

    // There are mutable methods as well, for eg:
    // iter() has iter_mut() as well
}
