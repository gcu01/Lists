use std::mem;
pub struct List{
    head: Link,
}
enum Link{
    Empty,
    More(Box<Node>),
}
struct Node {
    elem: i32,
    next: List,
}

impl List {
    pub fn new() -> Self{ 
        //todo!("working");
        List{head: Link::Empty,}
    }
    pub fn push(self: &mut Self, elem:i32) {
        let new_node = Box::new(Node {elem, next: mem::replace(&mut crate::first::List {head: self.head}, crate::first::List{head: Link::Empty})});
        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        todo!()
    }
}