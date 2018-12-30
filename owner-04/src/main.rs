fn main() {
    let x = vec![10, 20, 30];
    let b = true;
    if b {
        let y = x;
        println!("{:?}", y);
    } else {
        let z = x;
        println!("{:?}", z);
    }
    println!("{:?}", x); // いずれもxは移動する事が確定するのでコンパイルエラー
}
