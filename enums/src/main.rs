enum NavigationAids {
    VOR = 2,
    NDB = 3,
    VORDME = 5,
}

fn main() {
    println!("NDB:\t{}", NavigationAids::NDB as u8);
    println!("VOR:\t{}", NavigationAids::VOR as u8);
    println!("VORDME:\t{}", NavigationAids::VORDME as u8);
}
