use std::iter::Iterator;

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new = self.curr + self.next;

        self.curr = self.next;
        self.next = new;

        Some(self.curr)
    }
}

fn main() {
    let fib = Fibonacci { curr: 0, next: 1 };

    for i in fib.take(20) {
        println!("{}", i);
    }
}
