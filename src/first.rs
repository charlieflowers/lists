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
        match &mut self.head {
            Link::Empty => None,
            ref mut theNode @ Link::More(node) => {
                // STOPPED HERE. WTF DOES A MATCH STATEMENT NEED TO BORROW AT ALL??? Why can't it "just compare things"?
                // See, when there is a match, it lets you DESTRUCTURE IT and USE it. And you have to know what type that variable that
                //  you're going to use is (or those variables if you destrucutred it into small bits). And besides, should that target
                //  take ownership, or just take a reference? And of course, if it is a reference, then it is a POINTER, and you have
                //  to enforce all the immutability rules.
                //
                // So it's not just a Switch statement ... is also ASSIGNS one or more new variables based on your destructuring.
                //
                // Yes indeed, this is just another step in getting used to Rust's very foundational principle -- all aliasing to memory needs to be
                //  constrained.
                // A match, most of the time, is going to create BINDINGS. Bindings are just (in this case) local vars that give you a WINDOW INTO SOME
                //  MEMORY. And ANYTHING that gives you a window into some memory must follow the ownership and aliasing rules!
                //  How could it be otherwise, of course?
                // Could Rust be airtight about enforcing sane memory rules everywhere else, but "be cool" about match statements? Uh, what would
                //  be the point? Apps in it would have as many security holes and as much undefined behavior as something written in c/c++.
                // In a nutshell, Rust is special because it recognizes that ALL MEMORY is "global" and unsynchronized, and that is a formula for
                //  problems, and so Rust imposes some rules -- on ALL memory access -- that solve those problems.
                // Are you doing something that, in ANY way, interacts with memory? Whether you're reading it or mutating it? Well, then, you gotta
                //  know about and follow the rules.
                // Keep in mind, the reason match is nice and easy in Haskell is that Haskell is gratutiously making copies under the hood. AND YOU ARE
                //  COMPLETELY FREE TO DO THAT TOO OF COURSE! But you likely will shy away from it, because with some work you can figure out a more
                //  efficient way to solve it, and that is more in line with the culture of Rust. Similar to how you don't use "dynamic" in C#, but
                //  people often do use "Any" in Typescript -- even though they're the same thing.
                mem::replace(&mut self.head, node.next);
                Some(node.elem)
            }
        }
    }
}
