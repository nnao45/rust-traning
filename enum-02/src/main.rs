enum Select {
    A,
    B,
    C,
}

struct WithSelect {
    Name: String,
    Enum: Select,
}

fn a() {
    println!("Hello, A!");
}

fn b() {
    println!("Hello, B!");
}

fn c() {
    println!("Hello, C!");
}

fn ws(w :&WithSelect) {
    match w.Enum {
        Select::A => wa(w),
        Select::B => wb(w),
        Select::C => wc(w),
    }
}

fn wa(w :&WithSelect) {
    println!("{}", w.Name);
}

fn wb(w :&WithSelect) {
    println!("{}", w.Name);
}

fn wc(w :&WithSelect) {
    println!("{}", w.Name);
}

fn main() {
    let s: Select = Select::A;
    match s {
        Select::A => a(),
        Select::B => b(),
        Select::C => c(),
    }
    let w: WithSelect = WithSelect {
        Name: "I am B".to_string(),
        Enum: Select::B
    };
    ws(&w)
}
