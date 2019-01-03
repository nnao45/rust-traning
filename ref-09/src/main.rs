fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

fn main() {
    {
        let p = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&p);
        println!("{}", *s);
    }
}