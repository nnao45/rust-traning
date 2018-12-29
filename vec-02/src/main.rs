fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}

fn main() {
    let mut v = new_pixel_buffer(2, 3);
    println!("{:?}", v);
}
