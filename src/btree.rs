use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node {
    pub data: i32,
    pub left: Link,
    pub right: Link,
}

pub fn walk_pre_order(curr: Link, path: &mut Vec<i32>) -> &mut Vec<i32> {
    if let Some(node) = curr {
        path.push(node.borrow_mut().data);
        walk_pre_order(node.borrow_mut().left.clone(), path);
        walk_pre_order(node.borrow_mut().right.clone(), path);
    }
    path
}

pub fn walk_in_order(curr: Link, path: &mut Vec<i32>) -> &mut Vec<i32> {
    if let Some(node) = curr {
        walk_in_order(node.borrow_mut().left.clone(), path);
        path.push(node.borrow_mut().data);
        walk_in_order(node.borrow_mut().right.clone(), path);
    }
    path
}

pub fn walk_post_order(curr: Link, path: &mut Vec<i32>) -> &mut Vec<i32> {
    if let Some(node) = curr {
        walk_pre_order(node.borrow_mut().left.clone(), path);
        walk_post_order(node.borrow_mut().right.clone(), path);
        path.push(node.borrow_mut().data);
    }
    path
}

pub fn pre_order_search(head: Link) -> Vec<i32> {
    let mut path: Vec<i32> = Vec::new();
    walk_pre_order(head, &mut path);
    path
}

pub fn in_order_search(head: Link) -> Vec<i32> {
    let mut path: Vec<i32> = Vec::new();
    walk_in_order(head, &mut path);
    path
}

pub fn post_search(head: Link) -> Vec<i32> {
    let mut path: Vec<i32> = Vec::new();
    walk_post_order(head, &mut path);
    path
}
