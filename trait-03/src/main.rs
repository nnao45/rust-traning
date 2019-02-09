use std::io::{stdout, Write};
use std::fs::File;

fn say_hello(out: &mut Write) -> std::io::Result<()> {
    out.write_all(b"Hello World!");
    out.flush()
}

fn main() {
    let mut local_file = File::create("hello.txt");
    say_hello(&mut local_file);
}