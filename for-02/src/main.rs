fn main() {
    let strings: Vec<String> = vec!["Hai".to_string(), "Hoi".to_string(), "Hei".to_string()];
    let stringstrings: Vec<Vec<String>> = vec![strings.clone(), strings.clone(), strings.clone()];
    let mut i = 0;

    'kill:
    for ss in &stringstrings {
        for s in ss {
            i += 1;
            println!("String {:?} is at address {:p}", *s, s);
            if i == 5 {
                break 'kill;
            }
        }
    }    
}
