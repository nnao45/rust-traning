struct S<'abema> { // 生存期間の定義で使う英字は任意
    r: &'abema i32
}

fn main() {
    let s;
    {
        let x = 10;
        s = S {
            r : &x,
        };
        println!("{:?}", s.r);
    }
}
