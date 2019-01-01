fn g<'a>(p: &'a i32){
    println!("{:?}", p);
}

fn main() {
    let x = &10;
    g(x)
}
