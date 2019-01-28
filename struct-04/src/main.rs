struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { older: Vec::new(), younger: Vec::new()}
    }

    fn push(&mut self, c: T) {
        self.younger.push(c)
    }

    fn pop(&mut self) -> Self {
        if  self.older.is_empty() {
            if self.younger.is_empty() {
                return Option<T>
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
    //println!("{}", q.pop());

    q.push('σ');
    //println!("{}", q.pop());
    //println!("{}", q.pop());
    //println!("{}", q.pop());
}
