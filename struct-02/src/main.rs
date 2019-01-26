struct Bounds (usize, usize);

fn main() {
    let image_bounds = Bounds(1024, 768);
    println!("{:?}", image_bounds.0);
    println!("{:?}", image_bounds.1);
}
