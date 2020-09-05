use std::mem;

struct Node {
    elem: i32,
    next: Link,
}

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // match self.head {
        //     Link::Empty => None,
        //     // Link::More(ref mut node) => {
        //     //     mem::replace(&mut self.head, node.next);
        //     //     Some(node.elem)
        //     // }
        //     Link::More(ref mut node) => {
        //         mem::replace(&mut self.head, node.next);
        //         Some(node.elem)
        //     }
        // }

        let old_value = mem::replace(&mut self.head, Link::Empty);

        match old_value {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
