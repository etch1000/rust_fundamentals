fn main() {
    let scope_test = "Outer range";

    println!("{scope_test}");

    {
        let scope_test = "Inner range";
        println!("{scope_test}");
    }

    println!("{scope_test}");
}
