use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

// Binary Search Tree

#[derive(Debug, Clone)]
pub struct BSNode<T: Debug> {
    value: T,
    left: BLink<T>,
    right: BLink<T>,
}

type BLink<T> = Option<Rc<RefCell<BSNode<T>>>>;

impl<T: Debug> BSNode<T> {
    pub fn new(value: T, left: BLink<T>, right: BLink<T>) -> Self {
        Self { value, left, right }
    }
}

pub fn find<T: PartialEq + Ord + Copy + Debug>(n: &mut BLink<T>, value: T) -> bool {
    if n.is_none() {
        return false;
    }

    if let Some(item) = n {
        let v = item.clone();
        let mut r = Rc::try_unwrap(v)
            .ok()
            .expect("this didn't go well")
            .into_inner();
        if r.value == value {
            return true;
        }
        match r.value {
            v if v < value => return find(&mut r.right, value),
            v if v > value => return find(&mut r.left, value),
            _ => return false,
        }
    } else {
        return false;
    }
}

pub fn insert<T: PartialEq + Ord + Copy + Debug>(n: &mut BLink<T>, value: T) {
    if n.is_none() {
        return;
    }

    println!("inserting value {:?}", &value);
    if let Some(item) = n {
        //dbg!(&item);
        let v = item.clone();
        let mut i = Rc::try_unwrap(v)
            .ok()
            .expect("this didn't go well")
            .into_inner();
        match i.value {
            v if v < value && i.left.is_none() => {
                i.left = Some(Rc::new(RefCell::new(BSNode {
                    value,
                    left: None,
                    right: None,
                })))
            }
            v if v < value && i.right.is_none() => {
                i.right = Some(Rc::new(RefCell::new(BSNode {
                    value,
                    left: None,
                    right: None,
                })))
            }
            v if v < value => insert(&mut i.right, value),
            v if v > value => insert(&mut i.left, value),
            _ => (),
        }
    }
}

pub fn generate_bst<T: PartialEq + Ord + Copy>() -> BLink<i32> {
    // use rand::Rng;
    // let mut rng = rand::thread_rng();
    let mut bstree = Some(Rc::new(RefCell::new(BSNode::new(9, None, None))));
    insert(&mut bstree, 4);
    // for _ in 0..30 {
    //     let tmp = rng.gen_range(0..1000);
    //     dbg!(tmp);
    //     insert(&mut bstree, tmp);
    // }
    bstree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bst_is_built_properly() {
        let tree = generate_bst::<i32>();
        dbg!(tree);
    }
}
