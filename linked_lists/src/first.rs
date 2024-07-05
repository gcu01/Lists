use std::{mem};

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link{
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem:i32,
    next:Link,
}

impl List {
    pub fn new() -> Self {
        List{head: Link::Empty}
    }

    //push is adding a new elem on the head of the list
    // list             a->b->null   where a is the head
    // push(c) on list: c->a->b->null where c is the new head
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
                            elem:elem,
                            next:mem::replace(&mut self.head, Link::Empty),
                        });

    self.head = Link::More(new_node);
    }

    //pop is removing the elem from the head of the list
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => result = None,
            Link::More(node) => {
                self.head = node.next;
                result = Some(node.elem);
            }
        };
        //unimplemented!()
        result
    }
}







#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(None, list.pop());

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(2), list.pop());

        list.push(4);
        list.push(5);

        assert_eq!(Some(5), list.pop());
        list.pop();
        list.pop();

        assert_eq!(None, list.pop())

    }
}





//just some dummy code to understand that Self is the same as the type refered in the impl
/*
#[derive(Debug)]
pub struct Test {
    first:i32,
    second:i32,
}

impl Test {
    pub fn new() -> Test { //we can use -> Self (with capital S because it represents the type) as well, be carrefoul, Self is borrowing the value
        Test{first:0, second:0}
    }
}
*/