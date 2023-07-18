use std::collections::VecDeque;

#[derive(Default, Debug, Clone)]
pub struct BNode {
    value: i32,
    left: Option<Box<BNode>>,
    right: Option<Box<BNode>>,
}

impl BNode {
    pub fn new(value: i32, left: Option<Box<BNode>>, right: Option<Box<BNode>>) -> Self {
        Self { value, left, right }
    }
}

// Depth First search preserves shape
pub fn walk_btree_pre_order(curr: Option<Box<BNode>>, path: &mut Vec<Box<BNode>>) {
    if let Some(node) = curr {
        path.push(Box::new(*node.clone()));
        walk_btree_pre_order(node.left, path);
        walk_btree_pre_order(node.right, path);
    }
}

pub fn btree_pre_order_search(head: Option<Box<BNode>>) -> Vec<i32> {
    let mut path: Vec<Box<BNode>> = Vec::new();
    walk_btree_pre_order(head, &mut path);
    unimplemented!()
}

pub fn bfs(head: BNode, needle: i32) -> bool {
    let mut q = VecDeque::<BNode>::new();
    q.push_back(head);

    while q.len() > 0 {
        let curr = q.pop_front();
        match curr {
            Some(item) => {
                if item.value == needle {
                    return true;
                } else {
                    if item.left.is_some() {
                        q.push_back(*item.left.unwrap());
                    }
                    if item.right.is_some() {
                        q.push_back(*item.right.unwrap());
                    }
                }
            }
            None => {
                return false;
            }
        }
    }
    false
}

pub fn compare(a: Option<Box<BNode>>, b: Option<Box<BNode>>) -> bool {
    if a.is_none() && b.is_none() {
        return true;
    }

    if a.is_none() || b.is_none() {
        return false;
    }

    let a = a.unwrap();
    let b = b.unwrap();

    if a.value != b.value {
        return false;
    }

    return compare(a.left, b.left) && compare(a.right, b.right);
}
