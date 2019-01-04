use std::rc::Rc;

fn main() {
    let s: Rc<String> = Rc::new("Hi, Caphead!!".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
    println!("s:{} t:{} u:{}", s, t, u);
    println!("*s:{:p} *t:{:p} *u:{:p}", s, t, u);
}
