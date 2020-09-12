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
        // head is Option<Rc<Node<T>>>
        // Get the next node.
        // Drop the current node.
        // Repeat.

        // let _why_not_allowed = self.head.map(|_node| 42);

        // OK. We only have a mut ref to self. So we cannot get ownership damn it. Yet, try_unwrap needs ownership!
        // Ok, wait a minute. Maybe it is the indiana jones swap trick? Yes, of course. But ... how can THAT do it if we lack ownership? because, it uses unsafe code and swaps them!

        // let a = self.head.take();

        // let mut taken_head = self.head.take();
        // let mut cur_head = &self.head;

        // loop {
        //     if let Some(cur_node) = *cur_head {
        //         if let Ok(mut raw_node) = Rc::try_unwrap(cur_node) {
        //             // raw_node has OWNERSHIP, so when raw_node goes out of scope, it will be dropped!
        //             cur_head = &raw_node.next.take();
        //         }
        //     }
        // }

        // let mut current_head = self.head.take();

        // while let Some(mut cur_node) = self.head.take() {
        //     if let Ok(raw_node) = Rc::try_unwrap(cur_node) {
        //         // raw_node has OWNERSHIP, so when raw_node goes out of scope, it will be dropped!
        //         cur_node = raw_node.next;
        //     }

        // }

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

        let mut cur_head = &mut self.head;
        let mut next_head : Option<Rc<Node<T>>>;

        while cur_head.is_some() {
            next_head = helper(cur_head);
            cur_head = &mut next_head;
        }

        // // This WORKS! So I can do it once, but I can't turn it into a loop.
        // if let Some(cur_node) = self.head.take() {
        //     if let Ok(raw_node) = Rc::try_unwrap(cur_node) {
        //         // raw_node has OWNERSHIP, so when raw_node goes out of scope, it will be dropped!
        //         let next_node = raw_node.next;
        //     }
        // }

        // Yes. NOW we HAVE ownership!

        // if self.head.is_some() {
        //     let x = self.head.unwrap();
        //     let y = Rc::try_unwrap(x);

        //     if let Ok(mut unwrapped_node) = y {
        //         let _next_node = unwrapped_node.next;
        //         unwrapped_node.next = None;
        //     }
        // }

        // while let Some(cur_node) = self.head.as_ref() {
        //     if let Ok(mut unwrapped_node) = Rc::try_unwrap(cur_node) {

        //     }
        // }

        // OK, hold up. We need to get OWNERSHIP of the current node. Then, if there's only one count, we kill it.
        // self.head.as_ref().map(|node| {
        //     let x = Rc::try_unwrap(node);
        //     if let Ok(mut unwrapped_node) = x {
        //         let _next_node = unwrapped_node.next;
        //         unwrapped_node.next = None;
        //     }

        //     // Here, x and unwrapped_node will go out of scope. They now own the Rc, so it will be dropped.
        // });

        // if let Some(cur_node) = &self.head {
        //     let _next_node = cur_node.next.as_ref();
        //     let _wtf = *cur_node;
        //     let _x = Rc::try_unwrap(*cur_node); // I have a fucking ref to an Rc. I want to deref it to get an Rc. But that would MOVE the rc into try_unwrap, and the compiler says no.
        // }
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
