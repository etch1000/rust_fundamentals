fn main() {
    let duck: &str = "Duck";
    let airlines: &str = "Airlines";

    let airline_name = [duck, " ", airlines].concat();
    println!("{}", airline_name);

    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "every time";
    println!("{slogan}");
}
