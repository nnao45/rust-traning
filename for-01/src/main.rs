fn main() {
    let strings: Vec<String> = vec!["Hai".to_string(), "Hoi".to_string(), "Hei".to_string()];
    for s in &strings {
        println!("String {:?} is at address {:p}", *s, s);
    }
    println!("{:?}", strings);
}
