#[test]
fn math_words() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

fn main() {
    math_words()
}
