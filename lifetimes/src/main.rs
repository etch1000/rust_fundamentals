fn main() {
    // let outer_scope;

    // {
    //     let inner_scope = 5;
    //     outer_scope = &inner_scope;
    // }

    // println!("{outer_scope}");

    // let returned_reference = return_dangling_reference();
}

// fn return_dangling_reference() -> &i32 {
//     let value = 5;
//     &value
// }

fn lifetime_syntax<'a, 'b>(p1: &'a i32, p2: i32, p3: &'b f64) {}
