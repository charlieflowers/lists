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
        // Take ownership of the node in self.head
        // if it is none, return none
        // if it has a value, rest easy knowing it will be destroyed when you go out of scope
        // edit self.head to point to whatever node1.next points to
        // edit node2.prev to be None
        // Wait. I think you ALSO have to take ownership of n2.prev, which is ANOTHEr pointer to the node to kill. And you let that
        //  go out of scope too. That's how you reduce the reference count.

        let node_to_kill = self.head.take();

        match node_to_kill {
            None => None,
            Some(ntk) => {
                let n2 = ntk.borrow().next;
                self.head = n2;
                n2.map(|n| {
                    n.borrow().prev.take();
                });
                let x = *ntk;
                let y = x.get_mut();

                Some(std::mem::take(&mut y.elem)) // I returned to them an OWNED T. It is now theirs to control. Is that right?
            }
        }
    }
}
