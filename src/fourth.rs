use std::{cell::RefCell, rc::Rc};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, elem: T) {
        // Every node that comes into existence should, at all times, have 2 pointers pointing to it.
        //  When pushing a single node onto an empty list, the 2 pointers should be the list itself.
        //  I'm guessing a single node list should have the same head and tail.

        let new_node = Node::new(elem);

        if self.head.is_none() {
            // assert(tail.is_none()) because it should never be possible for head to be none without tail being none
            self.head = Some(new_node.clone()); // clone it here
            self.tail = Some(new_node.clone()); // clone it here
                                                // then let the original go out of scope, leaving 2 copies.
        } else {
            let old_head = self.head.take().unwrap();
            self.head = Some(new_node.clone());
            new_node.borrow_mut().next = Some(old_head.clone());
            old_head.borrow_mut().prev = Some(new_node.clone());
        }
    }
}
