pub struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

pub fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

pub fn example() {
    let f = fibonacci();
    for i in f.take(4) { // 4th next 수행
        println!("> {}", i);
    }
    for i in fibonacci().skip(4).take(4) { // skip 4th 수행
        println!("> {}", i);
    }
    for i in fibonacci().into_iter() { // IntoIter 반환
        if i > 10 {
            break;
        }
        println!("> {}", i);
    }
}