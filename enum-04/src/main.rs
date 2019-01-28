#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str{
        match self {
            TimeUnit::Seconds  => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }
    fn singular(self) -> &'static str {
        self.plural().trim_right_matches('s')
    }
}

fn main() {
    let m = TimeUnit::Minutes;
    println!("{:?}", m.plural());
    println!("{:?}", m.singular());
}
