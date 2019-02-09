pub struct Sink;

use std::io::{Write, Read};

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

fn main() {
    Sink::write();

    println!("{:?}", Sink::write());
}
