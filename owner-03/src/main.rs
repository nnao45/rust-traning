fn main() {
    let mut s = "Sousuke".to_string();
    let t = s; // Sousukeの所有権に移動
    s = "Ponyo".to_string(); // 移動後の変数には新たな値と所有権を与えることができる。
    println!("{}", s);
    println!("{}", t);
}
