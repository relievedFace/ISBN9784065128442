const MAX: usize = 10000;

struct Stack {
    st: [i32; MAX],
    top: usize,
}

impl Stack {
    fn new() -> Self {
        Stack {
            st: [0; MAX],
            top: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn is_full(&self) -> bool {
        self.top == MAX
    }

    fn push(&mut self, x: i32) {
        if self.is_full() {
            println!("error: stack is full.");

            return;
        }

        self.st[self.top] = x;
        self.top += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            println!("error: stack is empty.");

            None
        } else {
            self.top -= 1;

            Some(self.st[self.top])
        }
    }
}

fn main() {
    let mut stack = Stack::new();

    stack.push(3);
    stack.push(5);
    stack.push(7);

    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());

    stack.push(9);
}
