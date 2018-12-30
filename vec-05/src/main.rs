fn main() {
    let mut v = vec![10, 20, 30, 40, 50];
    v.insert(3, 35);
    println!("{:?}", v);
    v.remove(1);
    println!("{:?}", v);
    v.pop();
    println!("{:?}", v);
    v.pop();
    v.pop();
    v.pop();
    println!("{:?}", v);
    v.pop();
    println!("{:?}", v);
    v.pop();
    println!("{:?}", v);
}
