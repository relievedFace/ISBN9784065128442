struct Heap {
    heap: Vec<i32>,
}

impl Heap {
    fn new() -> Self {
        Heap { heap: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.heap.push(x);
        let mut i = self.heap.len() - 1;

        while i > 0 {
            let p = (i - 1) / 2;
            if self.heap[p] >= x {
                break;
            }
            self.heap[i] = self.heap[p];
            i = p;
        }

        self.heap[i] = x;
    }

    fn top(&self) -> Option<i32> {
        if !self.heap.is_empty() {
            Some(self.heap[0])
        } else {
            None
        }
    }

    fn pop(&mut self) {
        if let Some(x) = self.heap.pop() {
            let mut i = 0;

            while i * 2 + 1 < self.heap.len() {
                let mut child1 = i * 2 + 1;
                let child2 = i * 2 + 2;

                if child2 < self.heap.len() && self.heap[child2] > self.heap[child1] {
                    child1 = child2;
                }
                if self.heap[child1] <= x {
                    break;
                }
                self.heap[i] = self.heap[child1];
                i = child1;
            }

            self.heap[i] = x;
        }
    }
}

fn main() {
    let mut h = Heap::new();

    h.push(5);
    h.push(3);
    h.push(7);
    h.push(1);
    println!("{:?}", h.top());

    h.pop();
    println!("{:?}", h.top());

    h.push(11);
    println!("{:?}", h.top());
}
