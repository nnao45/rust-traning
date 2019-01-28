struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}


impl Queue {
    fn new() -> Queue {
        Queue { older: Vec::new(), younger: Vec::new()}
    }

    fn push(&mut self, c: char) {
        self.younger.push(c)
    }

    fn pop(&mut self) -> Option<char> {
        if  self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }
}

fn main() {
    let mut q = Queue::new();

    q.push('a');
    q.push('b');
    println!("{:?}", q.pop());

    q.push('σ');
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
}
