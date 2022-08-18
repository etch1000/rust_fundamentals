use std::ops::{Add, Sub};

fn main() {
    let sum = add(256, 262);
    println!("{sum}");
}

fn add<T>(op1: T, op2: T) -> T
where
    T: Add<Output = T> + Sub<Output = T>,
{
    op1 + op2
}
