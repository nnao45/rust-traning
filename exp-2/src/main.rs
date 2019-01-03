struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Some(Point {x: 0, y: 0 });
    m(&p)
}

fn m(p: &Option<Point>) {
    match p {
        Some(Point { x, y }) => println!("({},{})", x, y),
        None => println!("Nothing!!"),
    }
}