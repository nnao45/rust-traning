fn main() {
    let mut v1 = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    println!("{:?}", v1);
    let v2 = v1;
    // println!("{:?}", v1); 所有権はv2に移るから実行できない。
    println!("{:?}", v2);
    // let v3 = v1; 所有権はv2に移っているので実行できない。
    let v4 = v2.clone(); //clone()することで値のコピーをすることできる。
    println!("{:?}", v4);
}
 