pub struct List<T> {
    head: Link<T>,
}

// enum Link {
//     Empty,
//     More(Box<Node>),
// }
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            // next: mem::replace(&mut self.head, None),
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        // let old_value = mem::replace(&mut self.head, None);
        let old_value = self.head.take();

        // match old_value {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
        // }

        old_value.map(|node| {
            // See, you don't have to worry about the None branch. `map` knows to have that evaluate to None.
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem // This is crazy to me, but powerful. You're going to give out a ref to the guts of the node, but allow them to mutate it.
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // let mut cur_link = mem::replace(&mut self.head, None);
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            // cur_link = mem::replace(&mut boxed_node.next, None);
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here. HOWEVER, its Node has
            // had its `next` field set to Link::Empty, which means the compiler won't do
            // a bunch of recursive calls to `drop()`, and hence, no stack overflow.
        }
    }
}

// Here's how I made list iterable, without looking at their stuff. But they did it differently! Mine did work. Need to understand why they did it differently.
// pub struct ListIterator<T>(List<T>);

// impl<T> Iterator for List<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.pop()
//     }
// }

pub struct IntoIter<T>(List<T>);
impl<T> List<T> {
    // Note! You Can have more than one `impl` for the same struct!
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Alright, let's try to implement Iter. This time we won't be able to rely on List giving
// us all the features we want. We'll need to roll our own. The basic logic we want is to hold
// a pointer to the current node we want to yield next. Because that node may not exist (the
//     list is empty or we're otherwise done iterating), we want that reference to be an
//      Option. When we yield an element, we want to proceed to the current node's next
//      node.

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            // next: self.head.map(|node| node.as_ref()),
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn use_in_for() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut x = 42;

        for i in list.into_iter() {
            println!("Adding {} to {}", i, x);
            x += i;
        }

        assert_eq!(x, 48);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
