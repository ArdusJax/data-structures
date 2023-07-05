use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<SNode<T>>>>;

#[derive(Debug)]
pub struct SNode<T> {
    value: T,
    next: Link<T>,
}

impl<T> SNode<T> {
    pub fn new(item: T) -> Rc<RefCell<SNode<T>>> {
        Rc::new(RefCell::new(SNode {
            value: item,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct Stack<T> {
    pub length: i32,
    head: Link<T>,
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            length: 0,
            head: None,
        }
    }
    pub fn push(&mut self, item: T) {
        let node = SNode::new(item);
        match self.head.take() {
            Some(current) => {
                node.borrow_mut().next = Some(current.clone());
                self.head = Some(node);
                self.length += 1;
            }
            None => {
                self.head = Some(node.clone());
                self.length += 1;
            }
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                let h = Rc::try_unwrap(head)
                    .ok()
                    .expect("unable to destack")
                    .into_inner();
                self.head = h.next;
                self.length -= 1;
                Some(h.value)
            }
            _ => None,
        }
    }
    pub fn peek(mut self) -> Option<T> {
        if let Some(v) = self.head.take() {
            Some(
                Rc::try_unwrap(v)
                    .ok()
                    .expect("something went wrong")
                    .into_inner()
                    .value,
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_simple_stack() {
        let mut st: Stack<i32> = Stack::new();
        st.push(32);
        st.push(13);
        st.push(3);
        st.push(330);
        dbg!(st);
    }
    #[test]
    fn peek_simple_stack() {
        let mut st: Stack<i32> = Stack::new();
        st.push(32);
        st.push(13);
        st.push(3);
        st.push(330);
        let res = st.peek();
        dbg!(res);
    }
    #[test]
    fn push_simple_stack() {
        let mut st: Stack<i32> = Stack::new();
        st.push(32);
        st.push(13);
        st.push(3);
        st.push(330);
        dbg!(st);
    }
    #[test]
    fn pop_simple_stack() {
        let mut st: Stack<i32> = Stack::new();
        st.push(32);
        st.push(13);
        st.push(3);
        st.push(330);
        //pop a value off of the stack
        let res = st.pop();
        dbg!(st);
    }
}
// use std::borrow::{Borrow, BorrowMut};
// use std::cell::RefCell;
// use std::rc::Rc;
//
// type SLink<T> = Option<Rc<RefCell<SNode<T>>>>;
//
// #[derive(Debug)]
// pub struct SNode<T> {
//     value: T,
//     next: SLink<T>,
// }
//
// impl<T> SNode<T> {
//     pub fn new(item: T) -> Rc<RefCell<SNode<T>>> {
//         Rc::new(RefCell::new(SNode {
//             value: item,
//             next: None,
//         }))
//     }
// }
//
// #[derive(Debug)]
// pub struct Stack<T> {
//     pub length: i32,
//     head: SLink<T>,
//     tail: SLink<T>,
// }
//
// impl<T: Copy> Stack<T> {
//     pub fn enstack(&mut self, item: T) {
//         let node = Some(SNode::new(item));
//         match self.tail.take() {
//             Some(current) => {
//                 current.borrow_mut().next = node.clone();
//                 self.tail = node;
//                 self.length += 1;
//             }
//             None => {
//                 self.head = node.clone();
//                 self.tail = node;
//                 self.length += 1;
//             }
//         }
//     }
//     pub fn pop(&mut self) {
//         if let Some(head) = self.head.take() {
//             self.head = Rc::try_unwrap(head)
//                 .ok()
//                 .expect("unable to unwrap when popping")
//                 .into_inner()
//                 .next;
//             self.length -= 1;
//         }
//     }
//     pub fn peek(&mut self) -> Option<T> {
//         if let Some(head) = self.head.take() {
//             Some(
//                 Rc::try_unwrap(head)
//                     .ok()
//                     .expect("something went wrong")
//                     .into_inner()
//                     .value,
//             )
//         } else {
//             None
//         }
//     }
// }
