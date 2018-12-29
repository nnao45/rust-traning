fn main() {
    let ringo_text = "Ringo chan";
    println!("{}", ringo_text);
    let (head, tail) = ringo_text.split_at(6);
    println!("{}", head);
    assert_eq!(head, "Ringo ");
    println!("{}", tail);
    assert_eq!(tail, "chan");
}
