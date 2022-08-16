fn main() {
    // Modulus Operator
    let modulus = 18 % 7;
    println!("{modulus}");

    // Exponent Operator (or function more specifically)
    let cubed = i32::pow(4, 3);
    let float_integer = f32::powi(2.5, 4);
    let float_float = f32::powf(2.5, 3.14);
    println!("Integer: {cubed}");
    println!("Float to integer: {float_integer}");
    println!("Float to float: {float_float}");

    // Order of Operations : PEMDAS
    // Parantheses > Exponents > Multiplication > Division > Addition > Substraction
    let order_ops = 8 + 4 * 2 - (12 / 3 + 7) + 4;
    println!("Order of Operations result: {order_ops}");
    // Addition and Substraction happen left to right when there is no P,E,M or D.

    // Logical Operator
    let are_equal_is_true = 1 == 1;
    let are_equal_is_false = 1 == 2;
    let are_not_equal = 1 != 2;

    println!("are_equal_is_true: {are_equal_is_true}\nare_equal_is_false: {are_equal_is_false}\nare_not_equal: {are_not_equal}");

    let is_true = true;
    let is_false = !is_true;
    println!("is_true: {is_true}, is_false: {is_false}");

    // Logical AND
    // Result True
    let have_boarding_pass = true;
    let have_id = true;
    let can_board = have_boarding_pass && have_id;
    println!("Boarding Pass: {have_boarding_pass}, ID: {have_id}");
    println!("Can board plane: {can_board}");
    // Result False
    let have_boarding_pass = true;
    let have_id = false;
    let can_board = have_boarding_pass && have_id;
    println!("Boarding Pass: {have_boarding_pass}, ID: {have_id}");
    println!("Can board plane: {can_board}");

    // Logical OR
    // Result True
    let have_driver_license = false;
    let have_passport = true;
    let have_proof = have_driver_license || have_passport;
    let have_id = have_proof;
    let can_board = have_boarding_pass && have_id;
    println!("Boarding Pass: {have_boarding_pass}, ID: {have_id}");
    println!("Can board plane: {can_board}");
    // Result False
    let have_driver_license = false;
    let have_passport = false;
    let have_proof = have_driver_license || have_passport;
    let have_id = have_proof;
    let can_board = have_boarding_pass && have_id;
    println!("Boarding Pass: {have_boarding_pass}, ID: {have_id}");
    println!("Can board plane: {can_board}");

    // Greater Than and Less Than
    let first_value = 10;
    let second_value = 15;
    let results = first_value < second_value;
    println!("Results: {results}");
    let results = first_value > second_value;
    println!("Results: {results}");
}
