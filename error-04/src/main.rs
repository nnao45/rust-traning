use std::io::{self, BufRead};
use std::io::{self as other_io, BufReader};
use std::fs::File;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn read_numbers(file: &mut BufRead) -> GenResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

fn main() {
    let f = File::open("/repo/rust-traning/error-04/test.txt").expect("file read error");
    let mut f = BufReader::new(f);

    println!("{:?}", read_numbers(&mut f));

}
