fn main() {
    let mut v: Vec<isize> = Vec::with_capacity(2);
    println!("{}", v.len());
    println!("{}", v.capacity());
    v.push(1);
    v.push(2);
    println!("{:?}", v);
    println!("{}", v.len());
    println!("{}", v.capacity());
    v.push(3);
    println!("{:?}", v);
    println!("{}", v.len());
    println!("{}", v.capacity());
}
