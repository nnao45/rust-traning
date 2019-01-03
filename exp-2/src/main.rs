struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Some(Point {x: 0, y: 0 });
    m(&p1);
    let p2 = None;
    m(&p2);
}

fn m(p: &Option<Point>) {
    match p {
        Some(Point { x, y }) => println!("({},{})", x, y),
        None => println!("Nothing!!"),
    }
}