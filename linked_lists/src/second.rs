use std::{mem};

#[derive(Debug)]
pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    elem:i32,
    next:Link,
}

impl List {
    pub fn new() -> Self {
        List{head: None}
    }

    //push is adding a new elem on the head of the list
    // list             a->b->null   where a is the head
    // push(c) on list: c->a->b->null where c is the new head
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
                            elem:elem,
                            next:mem::replace(&mut self.head, None),
                        });

    self.head = Some(new_node);
    }

    //pop is removing the elem from the head of the list
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, None) {
            None => result = None,
            Some(node) => {
                self.head = node.next;
                result = Some(node.elem);
            }
        };
        //unimplemented!()
        result
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
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
