fn factional(n: usize) -> usize {
    n * (n + 1)
}

fn main() {
    let r = &factional(6);
    println!("fn: {}" ,r);
    println!("fn: {}" ,r);
}
