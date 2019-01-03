enum Alpha {
    A {x: String},
    B {y: isize},
    Quit,
}

fn mt(a: &Alpha) {
    match a {
        Alpha::A {x: ax} => println!("{}", ax),
        Alpha::B {y: by} => println!("{}", by),
        Alpha::Quit => quit(),
    }
}

fn quit() {
    return
}

fn main() {
    let a :Alpha = Alpha::A {x: "Hello, World!".to_string()};
    mt(&a);
    let b :Alpha = Alpha::B {y: 11111111111111};
    mt(&b);
    let q :Alpha = Alpha::Quit;
    mt(&q);
}
