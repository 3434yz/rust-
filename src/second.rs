use std::mem;

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug)]
struct List {
    head: Link,
}

impl List {
    fn new() -> Self {
        List { head: Link::Empty }
    }

    fn push(&mut self, elem: i32) {
        let box_node = Link::More(Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        }));

        self.head = box_node
    }

    fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::More(node) => {
                self.head = node.next;
                result = Some(node.elem);
            }
            Link::Empty => {
                result = None;
            }
        }
        result
    }
}

impl Drop for List {
    // add code here
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // while let Link::More(mut box_node) = cur_link {
        //     cur_link = mem::replace(&mut box_node.next,Link::Empty);
        // }
        loop {
            match cur_link {
                Link::More(mut box_node) => {
                    cur_link = mem::replace(&mut box_node.next, Link::Empty);
                }
                Link::Empty => {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        // assert_eq!(list.pop(), Some(5));
        // assert_eq!(list.pop(), Some(4));

        // // Check exhaustion
        // assert_eq!(list.pop(), Some(1));
        // assert_eq!(list.pop(), None);
    }
}
