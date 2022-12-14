// #[derive(Debug)]
// struct NdbNavAid {
//     name: String,
//     frequency: u16,
// }

// #[derive(Debug)]
// struct VorNavAid {
//     name: String,
//     frequency: f32,
// }

#[derive(Debug)]
struct NavAid<T, U> {
    name: String,
    frequency: T,
    data: U,
}

fn main() {
    let vor = NavAid {
        name: String::from("DQN"),
        frequency: 114.5,
        data: String::from("DQN is VOR"),
    };

    let ndb_data: Option<String> = Option::None;

    let ndb = NavAid {
        name: String::from("HKF"),
        frequency: 239,
        data: ndb_data,
    };

    println!("VOR information is: {:?}", vor);
    println!("NDB information is: {:?}", ndb);
}
