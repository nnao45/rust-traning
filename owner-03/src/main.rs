fn main() {
    let mut s = "Sousuke".to_string();
    let t = s; // Sousukeの所有権に移動
    s = "Ponyo".to_string();
    println!("{}", s);
    println!("{}", t);
}
