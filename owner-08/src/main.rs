fn main() {
    let str1 = "some".to_string();
    let str2 = str1;
    //println!("{}", str1); str1の所有権は移動すみ
    println!("{}", str2);

    let num1: i32 = 36;
    let num2 = num1;
    println!("{}", num1);
    println!("{}", num2);
}
