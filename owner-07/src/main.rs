struct Person {
    name: Option<String>,
    birth: i32,
}

fn main() {
    let mut composers = Vec::new();
    composers.push(Person { name: Some("Bob".to_string()),
                     birth: 1525,
                    });
    println!("{:?}: {:?}", composers[0].name, composers[0].birth);

    // let first_name = composers[0].name; インデックスされた所有権は移動できない。

    let first_name = composers[0].name.take(); // 0インデックス目のnameの参照先をNoneに置き換え、同時にfirst_nameに移動する。
    println!("first_name is {:?}", first_name); // first_nameに移動できる。
    println!("composers[0].name is {:?}", composers[0].name); // Noneが入っている。
}
