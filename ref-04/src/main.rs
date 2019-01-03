fn main() {
    let x = 10;
    let r = &x;
    println!("{:?}", *r); // 実態を見るためには*を使うべき
    println!("{:?}", r);  // しかし、Rustでは*を使わなくてもref変数に対して暗黙解決する。
}
