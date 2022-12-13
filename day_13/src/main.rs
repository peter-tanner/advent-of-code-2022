use std::ops::Deref;

struct Node {
    data: Option<u32>,
    children: Vec<Node>,
}

impl Node {
    fn new() -> Self {
        Node {
            data: None,
            children: Vec::new(),
        }
    }

    fn new_node(mut self) -> Box<Node> {
        let test = Box::from(Node::new());
        self.children.push(*test.deref());
        return test;
    }
}

fn main() {
    let list_s = "[1,[2,[3,[4,[5,6,7]]]],8,9]";

    let mut splitted = list_s.split(',').into_iter();

    let root = Node::new();

    while let token = splitted.next().unwrap() {
        if token.contains('[') {}
    }

    println!("Hello, world!");
}
