use std::mem::replace;

enum LinkedList {
    Node(String, Box<LinkedList>),
    Nil,
}

impl LinkedList {
    fn print_list(&self) {
        let mut current = self;
        while let LinkedList::Node(value, next) = current {
            print!("{} -> ", value);
            current = next;
        }

        println!();
    }

    fn insert(&mut self, value: String) {
        *self = LinkedList::Node(value, Box::new(replace(self, LinkedList::Nil)));
    }
}

fn main() {
    let mut linked_list = LinkedList::Nil;
    let names = [
        "yamamoto",
        "watanabe",
        "ito",
        "takahashi",
        "suzuki",
        "saito",
    ];

    for (i, name) in names.iter().enumerate() {
        linked_list.insert(name.to_string());

        print!("step {}: ", i);
        linked_list.print_list();
    }
}
