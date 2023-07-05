use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

pub struct Node {
    pub data: i32,
    pub prev: Link,
    pub next: Link,
}

impl Node {
    pub fn new(data: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn initialize_linked_list() {
        let new_linked_list = Node::new(0);
    }
}
