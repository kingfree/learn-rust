use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

// impl Drop for List {
//     fn drop(&mut self) {
//         // NOTE: you can't actually explicitly call `drop` in real Rust code;
//         // we're pretending to be the compiler!
//         self.head.drop(); // tail recursive - good!
//     }
// }

// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {} // Done!
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop(); // tail recursive - good!
//             }
//         }
//     }
// }

// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop(); // uh oh, not tail recursive!
//         deallocate(self.ptr);
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

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
}
