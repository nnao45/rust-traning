#![feature(termination_trait)]

use std::error::Error;

fn get_value_good<'a>(v: bool) -> Result<usize,&'a str> {
    if v {
        Ok(100)
    } else {
        Err("error message")
    }
}


fn main() -> Result<(), Box<Error>> {
    /* いちいち書いてられません！
    match get_value_good(true) {
        Ok(result) => println!("success: {}", result),
        Err(msg) => println!("failure: {}", msg),
    }
    match get_value_good(false) {
        Ok(result) => println!("success: {}", result),
        Err(msg) => println!("failure: {}", msg),
    }
    */
    let result1 = get_value_good(true)?;
    let result2 = get_value_good(false)?;
    Ok(())
}
