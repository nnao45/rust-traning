static mut STASH: &i32 = &128;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn main() {
    let p = &20;
    f(p);
    //println!("{:?}", STASH);
}
