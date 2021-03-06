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

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here. HOWEVER, its Node has
            // had its `next` field set to Link::Empty, which means the compiler won't do
            // a bunch of recursive calls to `drop()`, and hence, no stack overflow.
        }
    }
}

#[cfg(test)]
mod test {

    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    // Fascinating. the next step is to manually implement Drop because otherwise, recursive drop calls could cuaes a stack overflow for large data structures. I simply have
    //  never had to worry about that before. But now that I've looked into it, it makes complete sense. The default impl just doesn't work for you. The Rust Book doesn't do
    //  a good job of covering things like this and helping you understand what is now on your plate. But it all goes back to being in control of memory. I think maybe the
    //  Rust book is trying to make a false promise.
}
