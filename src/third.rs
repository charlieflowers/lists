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

// Question here! I want my iterator to simply use a List<T>. Each next item is head, and then it bumps its own position to tail. But
//  because List doesn't have any references in it, I cxannot seem to talk about its lifetime, and I need to say that Iter must live as
//  long as the List lives.
//
// Since I can't talk about List<T>'s lifetime, I'm forced to base my iterator on Option<Rc<Node<T>>> instead. Figure out if the other
// //  option does exist.
// pub struct Iter<'a, T>(List<T>)
// where
//     T: 'a;

// impl<'a, T> Iterator for Iter<'a, T>
// where
//     T: 'a,
// {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

pub struct Iter<'a, T> {
    next: &'a Option<Rc<Node<T>>>, // Aha! They DID do it on Option<&Node<T>>. So how did they "esccape from" the RC?
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next {
            self.next = &node.next;
            Some(&node.elem)
        } else {
            None
        }
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        // Iter { next: &self.head }

        let x = &self.head;
        let y = x.as_ref();
        let z = y.map(|node| &**node); // Aha! THAT's how you do it! You DEREF the Rc!!!

        let abc = y.map(|node| {
            let d = *node;
            let e = *d; // Yep! This is how you "get out of" an RC!! That's the missing piece I needed!
        })
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
