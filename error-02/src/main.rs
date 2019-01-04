fn get_value_good<'a>(v: bool) -> Result<usize,&'a str> {
    if v {
        Ok(100)
    } else {
        Err("error message")
    }
}


fn main() {
    match get_value_good(true) {
        Ok(result) => println!("success: {}", result),
        Err(msg) => println!("failure: {}", msg),
    }
    match get_value_good(false) {
        Ok(result) => println!("success: {}", result),
        Err(msg) => println!("failure: {}", msg),
    }
}
