use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (key, value) in table {
        println!("parent is {}", key);
        for v in value {
            println!(" {}", v);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("one".to_string(), vec!["one_one".to_string(), "two_one".to_string()]);
    table.insert("two".to_string(), vec!["one_two".to_string(), "two_two".to_string()]);
    table.insert("three".to_string(), vec!["one_three".to_string(), "two_three".to_string()]);
    
    show(&table);
    show(&table);
}
