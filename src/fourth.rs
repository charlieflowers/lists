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
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
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

    pub fn pop_front(&mut self) -> Option<T> {
        // OK, first, the high level requirements, THEN the pseudocode, then the code.
        // High level rqmts
        // Take: list.head
        // Take: n2.prev
        // Set: list.head = n2
        // That's it!
        // ---
        // pseudocode:
        // if list empty, return none
        // let ntk = self.head.take() // When this goes out of scope, one ref decreased
        // let n2 = ntk.next;
        // if n2.some()
        //      n2.prev.take()
        //      list.head = n2
        // return ntk.elem // tricky
        // That's it!

        let node_to_kill = self.head.take();

        match node_to_kill {
            None => None,
            Some(ntk) => {
                let n2 = &ntk.borrow().next;
                self.head = n2.clone();

                if let Some(inner_n2) = n2 {
                    inner_n2.borrow_mut().prev.take();
                }

                // The only problem is, I don't know what to put here!
                // But I bet they will cover this. They will probably require T to implement default.
                // A shame to require that, when I am only deleting things anyway.
                // I bet there is a way around that.
                let elem = std::mem::replace(ntk.borrow_mut().elem, 42); 

                Some(elem)
            }
        }
    }
}
