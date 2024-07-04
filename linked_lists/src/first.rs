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


}














//just some dummy code to understand that Self is the same as the type refered in the impl
/*
#[derive(Debug)]
pub struct Test {
    first:i32,
    second:i32,
}

impl Test {
    pub fn new() -> Test { //we can use -> Self as well, be carrefoul, Self is borrowing the value
        Test{first:0, second:0}
    }
}
*/