fn main() {
    let mut x: Vec<usize> = vec![10, 20, 30, 40, 50];
    let mut i = 10;
    while i > 0 { 
        println!("x is {:?}", x); //8行目で新しい値が入っているからセーフ。
        let y = x; // ここでyに所有権が移動
        println!("y is {:?}", y);
        x = vec![60, 70]; // 2周目で前にxに新しい値を入れておく。
        i -= 1;
    }
    println!("done.");
}
