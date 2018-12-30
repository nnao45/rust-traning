fn main() {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for i in 101..106 {
        v1.push(i.to_string());
        println!("{:?}", v1);
    }

    for mut s in v1 {
        s += &'!'.to_string();
        v2.push(s);
        println!("{:?}", v2);
    }
}
