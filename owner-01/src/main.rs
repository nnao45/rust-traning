struct Person {
    name: String,
    birth: i32,
}

fn main() {
    let mut vs = Vec::new();

    vs.push(Person {
        name: "Bob".to_string(),
        birth: 1525,
    });
    vs.push(Person {
        name: "Alice".to_string(),
        birth: 1825,
    });
    vs.push(Person {
        name: "John".to_string(),
        birth: 3525,
    });

    for v in &vs {
        println!("{}, born{}", v.name, v.birth);
    }
}
