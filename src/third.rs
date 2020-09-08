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
        // List {
        //     head: Some(Rc::new(Node {
        //         elem,
        //         next: self.head.as_ref().map(|node| node.clone()),
        //     })),
        // }
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        // I think I need to use and_then on the next line ... Rust's equivalent of flatmap
        // let mut head_node = &self.head.map(|node| &node.next.map(|n2| n2.clone())); // I have no fucking idea what is going on with the types here.

        // let mut one_step = &self.head.map(|node| &node.next);

        // Here's another line in which I have abso-fucking-lutely no fucking idea what is going on with the types
        //let use_this = self.head.and_then(|node| node.next).and_then(|node| node.next.clone());
        let use_this = self
            .head.as_ref()
            .and_then(|node| node.next.as_ref())
            .and_then(|node| node.next.clone());

        // Fuck it. I'm taking this to the playground, looking at the muir or whatever the fuck it takes to understand what is going on.

        List {
            head: use_this
        }

        // *******************************
        // This block works too. Cleaner in SOME ways, dirtier in others.
        // let mut head_node = None;

        // if let Some(outer_rc_node) = &self.head {
        //     let b = &outer_rc_node.next;
        //     head_node = b.as_ref().map(|rc_node| rc_node.clone());
        // }

        // List { head: head_node }

        // ************************
        // This block works
        // let a = &self.head;

        // let head_node = match a {
        //     None => None,
        //     Some(rc_node) => {
        //         let b = &rc_node.next;
        //         match b {
        //             None => None,
        //             Some(second_rc_node) => Some(second_rc_node.clone()),
        //         }
        //     }
        // };

        // List { head: head_node }
        // ************************

        // ************************
        // This block fails
        // let x = self.head.map(|node| node.next);

        // List {
        //     head: self.head.map(|node| node.next).map(|node| node.clone()),
        // }
        // ************************
    }
}
