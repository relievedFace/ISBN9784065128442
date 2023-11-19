const MAX: usize = 10000;

struct Queue {
    qu: [i32; MAX],
    tail: usize,
    head: usize,
}

impl Queue {
    fn new() -> Self {
        Queue {
            qu: [0; MAX],
            tail: 0,
            head: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&self) -> bool {
        self.head == (self.tail + 1) % MAX
    }

    fn enqueue(&mut self, x: i32) {
        if self.is_full() {
            println!("error: queue is full.");
        }

        self.qu[self.tail] = x;
        self.tail += 1;
        if self.tail == MAX {
            self.tail = 0;
        }
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            println!("error: queue is empty.");

            None
        } else {
            let res = self.qu[self.head];
            self.head += 1;
            if self.head == MAX {
                self.head = 0;
            }

            Some(res)
        }
    }
}
fn main() {
    let mut queue = Queue::new();

    queue.enqueue(3);
    queue.enqueue(5);
    queue.enqueue(7);

    println!("{:?}", queue.dequeue());
    println!("{:?}", queue.dequeue());

    queue.enqueue(9);
}
