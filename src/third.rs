use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.elem)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_ref().map(|n2| &**n2);
            &n.elem
        })
    }
}

impl<'a, T> List<T> {
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref().map(|n| &**n),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        fn helper<T>(cur_head: &mut Option<Rc<Node<T>>>) -> Option<Rc<Node<T>>> {
            if let Some(cur_node) = cur_head.take() {
                if let Ok(raw_node) = Rc::try_unwrap(cur_node) {
                    // raw_node has OWNERSHIP, so when raw_node goes out of scope, it will be dropped!
                    let next_node = raw_node.next;
                    return next_node;
                }
            }

            None
        }

        // Ah, I see. His loop structure was cleaner:
        // First, do take on the head
        // THEN start the loop, and 
        // ... take each `next`

        let mut cur_head = &mut self.head;
        let mut next_head: Option<Rc<Node<T>>>;

        while cur_head.is_some() {
            next_head = helper(cur_head);
            cur_head = &mut next_head;
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list: List<i32> = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list: List<i32> = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
