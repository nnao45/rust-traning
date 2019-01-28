use std::cmp::Ordering;
use std::cmp::Ordering::*;

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Less
    } else if n > m {
        Greater
    } else {
        Equal
    }
}

fn main() {
    println!("{:?}", compare(1, 2));
    println!("{:?}", compare(3, 2));
    println!("{:?}", compare(2, 2));
}
