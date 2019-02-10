struct Cat {
    name: String,
    kind: String,
}

impl Cat {
    fn cry(&self, cry_str: String) {
        println!("{} say, {}", &self.name, cry_str)
    }
}

fn main () {
    let cat = Cat {
        name: "JiJi".to_string(),
        kind: "Kuroneko".to_string(),
    };
    cat.cry("myaw".to_string());
}