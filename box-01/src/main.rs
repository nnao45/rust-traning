#![feature(box_syntax)]

struct BigStruct {
    one: i32,
    two: i32,
    // etc
    one_hundred: i32,
}

fn foo(x: Box<BigStruct>) -> BigStruct {
    *x
}

fn main() {
    let x = Box::new(BigStruct {
        one: 1,
        two: 2,
        one_hundred: 100,
    });
    println!("Struct x   is at address {:p}", x);
    println!("String {:?}   is at address {:p}", x.one, &x.one);
    println!("String {:?}   is at address {:p}", x.two, &x.two);
    println!("String {:?} is at address {:p}", x.one_hundred, &x.one_hundred);

    let y: Box<BigStruct> = box foo(x);
    println!("Struct y   is at address {:p}", y);
    println!("String {:?}   is at address {:p}", y.one, &y.one);
    println!("String {:?}   is at address {:p}", y.two, &y.two);
    println!("String {:?} is at address {:p}", y.one_hundred, &y.one_hundred);
}
